use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, thread};

use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, RemoveContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::{ContainerStateStatusEnum, HostConfig, PortBinding};
use bollard::Docker;
use futures_util::stream::StreamExt;

use crate::common::config::CONFIG;
use crate::common::connect_to_netconf;

#[cfg(unix)]
pub struct NCServer {
    docker: Docker,
}

impl NCServer {
    pub fn new() -> Self {
        NCServer {
            docker: Docker::connect_with_socket_defaults().unwrap(),
        }
    }

    pub async fn start(&self) {
        self.start_container().await;
        self.wait_for_running_state().await;
        self.wait_for_netconf_ready();
        self.load_model().await;
        self.wait_for_netconf_ready();
        self.import_data().await;
        self.wait_for_netconf_ready();
    }

    pub async fn stop(&self) {
        self.stop_container().await;
    }

    fn create_container_config(&self) -> Config<String> {
        let exposed_port = CONFIG.netconf.port.to_string() + "/tcp";
        let exposed_ports: HashMap<_, _> = vec![(exposed_port.clone(), HashMap::<(), ()>::new())]
            .into_iter()
            .collect();

        let port_bindings: HashMap<_, _> = vec![(
            exposed_port,
            Some(vec![PortBinding {
                host_ip: Some(CONFIG.netconf.host.clone()),
                host_port: Some(CONFIG.netconf.port.to_string()),
            }]),
        )]
        .into_iter()
        .collect();

        let image = CONFIG.netconf.docker_image.clone() + "@sha256:" + &CONFIG.netconf.docker_sha;

        Config {
            image: Some(image),
            tty: Some(true),
            exposed_ports: Some(exposed_ports),
            host_config: Some(HostConfig {
                port_bindings: Some(port_bindings),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    async fn start_container(&self) {
        let options = Some(CreateContainerOptions {
            name: &CONFIG.netconf.container_name[..],
        });

        let config = self.create_container_config();

        let id = self
            .docker
            .create_container::<&str, String>(options, config)
            .await
            .unwrap()
            .id;

        self.docker
            .start_container::<String>(&id, None)
            .await
            .unwrap();
    }

    async fn wait_for_running_state(&self) {
        let options = Some(InspectContainerOptions { size: false });

        loop {
            let resp = self
                .docker
                .inspect_container(&CONFIG.netconf.container_name, options)
                .await
                .unwrap();

            if let Some(state) = resp.state {
                if let Some(status) = state.status {
                    if status == ContainerStateStatusEnum::RUNNING {
                        break;
                    }
                }
            }

            thread::sleep(Duration::from_secs(1))
        }
    }

    fn wait_for_netconf_ready(&self) {
        loop {
            if let Ok(_) = connect_to_netconf(
                &CONFIG.netconf.host,
                CONFIG.netconf.port,
                &CONFIG.netconf.user,
                &CONFIG.netconf.password,
            ) {
                break;
            }
        }
    }

    async fn load_model(&self) {
        let mut test_model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_model_path.push("tests/resources/test_model.yang");

        let mut create_test_model_cmd = String::from("echo '");
        create_test_model_cmd.push_str(&fs::read_to_string(test_model_path).unwrap());
        create_test_model_cmd.push_str("' > /tmp/test_model.yang");

        let install_yang_cmd =
            r#"sysrepoctl --install /tmp/test_model.yang && pkill -f netopeer2-server"#;
        let config = CreateExecOptions {
            cmd: Some(vec![
                "bash".to_string(),
                "-c".to_string(),
                create_test_model_cmd.to_string() + " && " + install_yang_cmd,
            ]),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            tty: Some(true),
            ..Default::default()
        };

        let exec = self
            .docker
            .create_exec(&CONFIG.netconf.container_name, config)
            .await
            .unwrap()
            .id;

        if let StartExecResults::Attached { mut output, .. } =
            self.docker.start_exec(&exec, None).await.unwrap()
        {
            while let Some(out_data) = output.next().await {
                if let Err(data) = out_data {
                    eprintln!("Exec error {:#?}", data);
                }
            }
        }
    }

    async fn import_data(&self) {
        let mut init_data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        init_data_path.push("tests/resources/init_data.xml");

        let mut create_init_data_cmd = String::from("echo '");
        create_init_data_cmd.push_str(&fs::read_to_string(init_data_path).unwrap());
        create_init_data_cmd.push_str("' > /tmp/init_data.xml");

        let import_init_data_cmd =
            r#"sysrepocfg --import=/tmp/init_data.xml --datastore startup --module test"#;
        let config = CreateExecOptions {
            cmd: Some(vec![
                "bash".to_string(),
                "-c".to_string(),
                create_init_data_cmd.to_string() + " && " + import_init_data_cmd,
            ]),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            tty: Some(true),
            ..Default::default()
        };

        let exec = self
            .docker
            .create_exec(&CONFIG.netconf.container_name, config)
            .await
            .unwrap()
            .id;

        if let StartExecResults::Attached { mut output, .. } =
            self.docker.start_exec(&exec, None).await.unwrap()
        {
            while let Some(out_data) = output.next().await {
                if let Err(data) = out_data {
                    eprintln!("Exec error {:#?}", data);
                }
            }
        }
    }

    async fn stop_container(&self) {
        let options = Some(RemoveContainerOptions {
            force: true,
            v: true,
            ..Default::default()
        });

        self.docker
            .remove_container(&CONFIG.netconf.container_name[..], options)
            .await
            .unwrap();
    }
}

use common::config::CONFIG;
use common::{run_test, setup_client};
use netconf_client::errors::NetconfClientError::*;
use netconf_client::models::{replies::*, requests::*};
use netconf_client::netconf_client::NetconfClient;
use serial_test::serial;

mod common;

#[test]
#[serial]
fn connect() {
    run_test(|| {
        let mut client = NetconfClient::new(
            &CONFIG.netconf.host,
            CONFIG.netconf.port,
            &CONFIG.netconf.user,
            &CONFIG.netconf.password,
        );
        let rsp = client.connect().unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
        client.send_hello().unwrap();
    });
}

#[test]
#[serial]
fn close_session() {
    run_test(|| {
        let mut client = setup_client();
        let rsp = client.close_session().unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
    });
}

#[test]
#[serial]
fn kill_session() {
    run_test(|| {
        let mut client = setup_client();
        let client2 = setup_client();

        let rsp = client
            .kill_session(client2.get_session_id().unwrap())
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
    });
}

#[test]
#[serial]
fn lock_unlock() {
    run_test(|| {
        let mut client = setup_client();
        let mut client2 = setup_client();

        let rsp = client.lock(DatastoreType::Running).unwrap();
        assert!(rsp.is_ok());

        let rsp = client2.lock(DatastoreType::Running).unwrap_err();
        match rsp {
            NetconfError { err: errors } => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].error_type.value, ErrorTypeE::Protocol);
                assert_eq!(errors[0].error_tag.value, ErrorTagE::LockDenied);
                assert_eq!(errors[0].error_severity.value, ErrorSeverityE::Error);
            }
            _ => panic!("Wrong error type {:#?}", rsp),
        }

        let rsp = client2.unlock(DatastoreType::Running).unwrap_err();
        match rsp {
            NetconfError { err: errors } => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].error_type.value, ErrorTypeE::Protocol);
                assert_eq!(errors[0].error_tag.value, ErrorTagE::LockDenied);
                assert_eq!(errors[0].error_severity.value, ErrorSeverityE::Error);
            }
            _ => panic!("Wrong error type {:#?}", rsp),
        }

        let rsp = client.unlock(DatastoreType::Running).unwrap();
        assert!(rsp.is_ok());
    });
}

#[test]
#[serial]
fn get() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .edit_config(
                DatastoreType::Running,
                r#"<users xmlns="ns:yang:test"><name>Harry</name></users>"#.to_string(),
            )
            .unwrap();
        assert!(rsp.is_ok());

        let rsp = client
            .get(Some(Filter {
                filter_type: FilterType::Subtree,
                data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
            }))
            .unwrap();
        assert!(rsp.is_ok());
        assert_eq!(
            rsp.data,
            Some("<users xmlns=\"ns:yang:test\"><name>Harry</name></users>".to_string())
        );
    });
}

#[test]
#[serial]
fn edit_config_running_database() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .edit_config(
                DatastoreType::Running,
                r#"<users xmlns="ns:yang:test"><name>Bob</name></users>"#.to_string(),
            )
            .unwrap();
        assert!(rsp.is_ok());

        let rsp = client
            .get_config(
                DatastoreType::Running,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok());
        assert_eq!(
            rsp.data,
            Some("<users xmlns=\"ns:yang:test\"><name>Bob</name></users>".to_string())
        );
    });
}

#[test]
#[serial]
fn edit_config_copy_config() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .edit_config(
                DatastoreType::Running,
                r#"<users xmlns="ns:yang:test"><name>Emily</name></users>"#.to_string(),
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);

        let rsp = client
            .copy_config(
                DatastoreType::Startup,
                CopyConfigSourceType::Datastore {
                    source: DatastoreType::Running,
                },
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);

        let rsp = client
            .get_config(
                DatastoreType::Startup,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
        assert_eq!(
            rsp.data,
            Some("<users xmlns=\"ns:yang:test\"><name>Emily</name></users>".to_string())
        );

        let rsp = client.delete_config(DatastoreType::Startup).unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);

        let rsp = client
            .get_config(
                DatastoreType::Startup,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
        assert_eq!(rsp.data, Some("".to_string()));
    });
}

#[test]
#[serial]
fn delete_startup_data() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .get_config(
                DatastoreType::Startup,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
        assert_eq!(
            rsp.data,
            Some(r#"<users xmlns="ns:yang:test"><name>Harry</name></users>"#.to_string())
        );

        let rsp = client.delete_config(DatastoreType::Startup).unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);

        let rsp = client
            .get_config(
                DatastoreType::Startup,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok(), "{:#?}", rsp);
        assert_eq!(rsp.data, Some("".to_string()));
    });
}

#[test]
#[serial]
fn edit_config_candidate_then_commit() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .edit_config(
                DatastoreType::Candidate,
                r#"<users xmlns="ns:yang:test"><name>Alice</name></users>"#.to_string(),
            )
            .unwrap();
        assert!(rsp.is_ok());

        let rsp = client.commit().unwrap();
        assert!(rsp.is_ok());

        let rsp = client
            .get_config(
                DatastoreType::Running,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok());
        assert_eq!(
            rsp.data,
            Some("<users xmlns=\"ns:yang:test\"><name>Alice</name></users>".to_string())
        );
    });
}

#[test]
#[serial]
fn discard_changes() {
    run_test(|| {
        let mut client = setup_client();

        let rsp = client
            .edit_config(
                DatastoreType::Candidate,
                r#"<users xmlns="ns:yang:test"><name>Lily</name></users>"#.to_string(),
            )
            .unwrap();
        assert!(rsp.is_ok());

        let rsp = client.discard_changes().unwrap();
        assert!(rsp.is_ok());

        let rsp = client
            .get_config(
                DatastoreType::Running,
                Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: r#"<users xmlns="ns:yang:test"></users>"#.to_string(),
                }),
            )
            .unwrap();
        assert!(rsp.is_ok());
        assert_eq!(rsp.data, Some("".to_string()));
    });
}

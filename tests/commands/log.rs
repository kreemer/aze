use assert_cmd::prelude::*;

use chrono::{Local, NaiveDateTime};
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;

use super::{add_frame, add_frame_with_tags};

#[test]
fn nothing_if_no_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn entry_from_this_day() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 3600;
    let end = start + 1800;

    let dt_start = NaiveDateTime::from_timestamp(start, 0);
    let dt_end = NaiveDateTime::from_timestamp(end, 0);

    add_frame(&database.to_str().unwrap(), &"test", &dt_start, &dt_end)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0h 30m 00s"));

    Ok(())
}

#[test]
fn entry_from_last_two_weeks_default_not_shown() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 3600 * 24 * 10;
    let end = start + 1800;

    let dt_start = NaiveDateTime::from_timestamp(start, 0);
    let dt_end = NaiveDateTime::from_timestamp(end, 0);

    add_frame(&database.to_str().unwrap(), &"test", &dt_start, &dt_end)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn duration_correctly_calculated() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame(
        &database.to_str().unwrap(),
        &"test",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0h 30m 00s"))
        .stdout(predicate::str::contains("1h 00m 00s"));

    Ok(())
}

#[test]
fn get_only_correct_project_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn get_only_correct_projects_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1")
        .arg("-p")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}

#[test]
fn ignore_single_project() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-project")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_multiple_projects() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-project")
        .arg("test1")
        .arg("--ignore-project")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1").not())
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_and_select_projects() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1")
        .arg("--ignore-project")
        .arg("test1");

    cmd.assert().failure().stderr(predicate::str::contains(
        "given projects can't be ignored at the same time",
    ));

    Ok(())
}

#[test]
fn get_only_correct_tag_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
        vec!["test1".to_string()],
    )?;
    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
        vec!["test2".to_string()],
    )?;
    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn get_only_correct_tags_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
        vec!["test1".to_string()],
    )?;
    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
        vec!["test2".to_string()],
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1")
        .arg("-T")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}

#[test]
fn ignore_single_tag() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
    )?;
    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
        vec!["test2".to_string()],
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-tag")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_multiple_tags() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
        vec!["test1".to_string()],
    )?;
    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
        vec!["test2".to_string()],
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-tag")
        .arg("test1")
        .arg("--ignore-tag")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1").not())
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_and_select_tags() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1")
        .arg("--ignore-tag")
        .arg("test1");

    cmd.assert().failure().stderr(predicate::str::contains(
        "given tags can't be ignored at the same time",
    ));

    Ok(())
}

#[test]
fn get_entries_from_multiple_tags() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        &NaiveDateTime::from_timestamp(end, 0),
        vec!["test1".to_string(), "test3".to_string()],
    )?;
    add_frame_with_tags(
        &database.to_str().unwrap(),
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        &NaiveDateTime::from_timestamp(end + 3600, 0),
        vec!["test2".to_string(), "test3".to_string()],
    )?;
    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test3");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}
use std::time::Instant;

#[test]
#[attr_wrapper::time_me]
fn default() {

}

#[test]
#[attr_wrapper::time_me(20ms)]
fn specific() {

}

#[test]
#[attr_wrapper::time_me(0)]
fn verbose() {

}
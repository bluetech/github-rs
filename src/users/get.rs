//! Access the Users portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::Executor;
use Json;

// Declaration of types representing the various items under users
new_type!(Emails);
new_type!(Followers);
new_type!(Following);
new_type!(FollowingUser);
new_type!(Keys);
new_type!(KeysId);
new_type!(Orgs);
new_type!(User);
new_type!(Users);
new_type!(Events);
new_type!(UsersKeys);
new_type!(UserUsername);
new_type!(UsersUsername);
new_type!(Repos);
new_type!(Issues);
new_type!(Starred);
new_type!(StarredRepo);
new_type!(StarredOwner);
new_type!(Subscriptions);

// From implementations for conversion
from!(Emails, Executor);
from!(Events, Executor);
from!(Followers, Executor);
from!(Following, FollowingUser);
from!(Following, Executor);
from!(FollowingUser, Executor);
from!(Issues, Executor);
from!(Keys, KeysId);
from!(Keys, Executor);
from!(KeysId, Executor);
from!(Orgs, Executor);
from!(Subscriptions, Executor);
from!(Starred, Executor);
from!(Starred, StarredOwner);
from!(StarredOwner, StarredRepo);
from!(StarredRepo, Executor);
from!(User, Emails, "emails");
from!(User, Followers, "followers");
from!(User, Following, "following");
from!(User, Keys, "keys");
from!(User, Executor);
from!(User, Issues, "issues");
from!(User, Orgs, "orgs");
from!(User, Subscriptions, "subscriptions");
from!(User, Starred, "starred");
from!(Users, Executor);
from!(Users, UsersUsername);
from!(UsersKeys, Executor);
from!(UserUsername, Followers, "followers");
from!(UserUsername, Following, "following");
from!(UserUsername, UsersKeys, "keys");
from!(UserUsername, Repos, "repos");
from!(UserUsername, Executor);
from!(UsersUsername, Followers, "followers");
from!(UsersUsername, Following, "following");
from!(UsersUsername, Events, "events");
from!(UsersUsername, UsersKeys, "keys");
from!(UsersUsername, Repos, "repos");
from!(UsersUsername, Executor);
from!(User, Repos, "repos");
from!(Repos, Executor);

// impls of each type
impl<'a> Starred<'a> {
    func!(owner, StarredOwner, owner_str);
    exec!();
}

impl<'a> StarredOwner<'a> {
    func!(repo, StarredRepo, repo_str);
}

impl<'a> User<'a> {
    func!(emails, Emails);
    func!(followers, Followers);
    func!(following, Following);
    func!(issues, Issues);
    func!(repos, Repos);
    func!(subscriptions, Subscriptions);
    func!(starred, Starred);
    func!(keys, Keys);
    func!(orgs, Orgs);
    exec!();
}

impl<'a> Users<'a> {
    func!(username, UsersUsername, username_str);
    exec!();
}

impl<'a> UserUsername<'a> {
    func!(followers, Followers);
    func!(following, Following);
    func!(keys, UsersKeys);
    func!(repos, Repos);
    exec!();
}

impl<'a> UsersUsername<'a> {
    func!(events, Events);
    func!(followers, Followers);
    func!(following, Following);
    func!(keys, UsersKeys);
    func!(repos, Repos);
    exec!();
}

impl<'a> Events<'a> {
    exec!();
}

impl<'a> Keys<'a> {
    // This is a status based call, will need to figure out
    func!(id, KeysId, id_str);
    exec!();
}

impl<'a> Following<'a> {
    // This is a status based call, will need to figure out
    func!(username, Following, username_str);
    exec!();
}

exec!(UsersKeys);
exec!(Emails);
exec!(FollowingUser);
exec!(Issues);
exec!(KeysId);
exec!(Followers);
exec!(Repos);
exec!(Subscriptions);
exec!(StarredRepo);
exec!(Orgs);

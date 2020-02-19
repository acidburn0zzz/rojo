//! Defines the semantics that Rojo uses to turn entries on the filesystem into
//! Roblox instances using the instance snapshot subsystem.

#![allow(dead_code)]

// mod csv;
mod dir;
mod error;
// mod json_model;
// mod lua;
// mod meta_file;
mod middleware;
// mod project;
// mod rbxlx;
// mod rbxm;
// mod rbxmx;
// mod txt;
// mod user_plugins;
// mod util;

pub use self::error::*;

use std::path::Path;

use rbx_dom_weak::{RbxId, RbxTree};
use vfs::Vfs;

// csv::SnapshotCsv,
// json_model::SnapshotJsonModel,
// lua::SnapshotLua,
// project::SnapshotProject,
// rbxlx::SnapshotRbxlx,
// rbxm::SnapshotRbxm,
// rbxmx::SnapshotRbxmx,
// txt::SnapshotTxt,
// user_plugins::SnapshotUserPlugins,
use self::dir::SnapshotDir;
use self::middleware::{SnapshotInstanceResult, SnapshotMiddleware};
use crate::{project::ProjectNode, snapshot::InstanceContext};

// pub use self::project::snapshot_project_node;

pub fn snapshot_project_node(
    context: &InstanceContext,
    project_folder: &Path,
    instance_name: &str,
    node: &ProjectNode,
    vfs: &Vfs,
) -> SnapshotInstanceResult {
    unimplemented!()
}

macro_rules! middlewares {
    ( $($middleware: ident,)* ) => {
        /// Generates a snapshot of instances from the given path.
        pub fn snapshot_from_vfs(
            context: &InstanceContext,
            vfs: &Vfs,
            path: &Path,
        ) -> SnapshotInstanceResult {
            $(
                log::trace!("trying middleware {} on {}", stringify!($middleware), path.display());

                if let Some(snapshot) = $middleware::from_vfs(context, vfs, path)? {
                    log::trace!("middleware {} success on {}", stringify!($middleware), path.display());
                    return Ok(Some(snapshot));
                }
            )*

            log::trace!("no middleware returned Ok(Some)");
            Ok(None)
        }
    };
}

middlewares! {
    // SnapshotProject,
    // SnapshotUserPlugins,
    // SnapshotJsonModel,
    // SnapshotRbxlx,
    // SnapshotRbxmx,
    // SnapshotRbxm,
    // SnapshotLua,
    // SnapshotCsv,
    // SnapshotTxt,
    SnapshotDir,
}

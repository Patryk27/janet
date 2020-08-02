use crate::gitlab::{GitLabClient, ProjectId};
use crate::interface::{ProjectPtr, PtrContext};
use anyhow::*;

impl ProjectPtr {
    pub async fn resolve(&self, gitlab: &GitLabClient, ctxt: &PtrContext) -> Result<ProjectId> {
        log::debug!("Resolving project ptr: {:?}", self);

        (try {
            match self {
                Self::Id(id) => *id,

                Self::Name { namespace, name } => {
                    let namespace = if let Some(namespace) = namespace {
                        namespace.resolve(gitlab).await?
                    } else {
                        ctxt.namespace_id
                            .ok_or_else(|| anyhow!("Cannot infer namespace id"))?
                    };

                    let namespace = gitlab.namespace(namespace.inner().to_string()).await?;

                    gitlab
                        .project(format!("{}/{}", namespace.full_path, name.as_ref()))
                        .await?
                        .id
                }
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve project ptr: {:?}", self))
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     mod resolve {
//         use super::*;
//
//         mod given_ptr_with_id {
//             use super::*;
//
//             #[tokio::test(threaded_scheduler)]
//             async fn returns_it() {
//                 let gitlab = GitLabClient::mock();
//
//                 let actual = ProjectPtr::Id(ProjectId::new(123))
//                     .resolve(&gitlab)
//                     .await
//                     .unwrap();
//
//                 let expected = ProjectId::new(123);
//
//                 assert_eq!(expected, actual);
//             }
//         }
//
//         // mod given_ptr_with_name {
//         //     use super::*;
//         //
//         //     #[tokio::test(threaded_scheduler)]
//         //     async fn transforms_name_into_id() {
//         //         let gitlab = GitLabClient::mock();
//         //
//         //         let actual =
//         // ProjectPtr::Name(ProjectName::new("test-project"))
//         //             .resolve(&gitlab)
//         //             .await
//         //             .unwrap();
//         //
//         //         let expected = ProjectId::new(123);
//         //
//         //         assert_eq!(expected, actual);
//         //     }
//         // }
//     }
// }

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::SdkConfig;
use aws_sdk_ec2::{Endpoint, Region};
use aws_sdk_sts::Client as StsClient;
use pubsys_config::AwsConfig;
use snafu::{OptionExt, ResultExt};

pub(crate) async fn build_client_config<S1, S2>(
    region: S1,
    sts_region: S2,
    aws: &AwsConfig,
) -> SdkConfig
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut config = aws_config::from_env();
    let region = region.as_ref().to_owned();

    // Add profile credential provider if specified in aws.region.REGION.role.
    let maybe_regional_role = aws.region.get(&region).and_then(|r| r.role.clone());
    let assume_roles = aws.role.iter().chain(maybe_regional_role.iter()).cloned();
    //    config = if let Some(profile) = &maybe_regional_role {
    //        let provider = build_provider(sts_region, assume_roles.clone(), &aws.profile);
    //        config.credentials_provider(provider)
    //    } else {
    //        config
    //    };

    // Add custom endpoint if specified in aws.region.REGION.endpoint.
    let maybe_regional_endpoint = aws.region.get(&region).and_then(|r| r.endpoint.clone());
    config = if let Some(endpoint) = &maybe_regional_endpoint {
        config.endpoint_resolver(Endpoint::immutable(endpoint.parse().expect("valid URI")))
    } else {
        config
    };

    config.region(Region::new(region)).load().await
}

// async fn build_provider<S>(
//     sts_region: S,
//     assume_roles: impl Iterator<Item = String>,
//     maybe_profile: &Option<String>,
// ) -> Result<?????>
// where
//     S: AsRef<str>,
// {
//     let mut config = aws_config::from_env();
//
//     // If the user specified a profile, use that, otherwise use the default
//     // credentials mechanisms.
//     config = if let Some(profile) = maybe_profile {
//         config.credentials_provider(
//             ProfileFileCredentialsProvider::builder()
//                 .profile_name(profile)
//                 .build(),
//         )
//     } else {
//         config
//     };
//
//     let sts_config = config.region(Region::new(sts_region.as_ref())).load().await;
//
//     for assume_role in assume_roles {
//         let sts_client = StsClient::new(&sts_config);
//         let credentials = sts_client
//             .assume_role()
//             .role_arn(assume_role)
//             .role_session_name("pubsys")
//             .send()
//             .await
//             .context(error::AssumeRoleSnafu {
//                 role_arn: assume_role,
//             })?
//             .credentials()
//             .context(error::CredentialsMissingSnafu {
//                 role_arn: assume_role,
//             })?
//             .clone();
//         provider = ?????;
//     }
//
//     Ok(provider)
// }
//
//pub(crate) mod error {
//
//    use aws_sdk_sts::error::AssumeRoleError;
//    use aws_sdk_sts::types::SdkError;
//    use snafu::Snafu;
//
//    #[derive(Debug, Snafu)]
//    #[snafu(visibility(pub))]
//    #[allow(clippy::large_enum_variant)]
//    pub(crate) enum Error {
//        AssumeRole {
//            role_arn: String,
//            source: SdkError<AssumeRoleError>,
//        },
//
//        #[snafu(display("Credentials were missing for assumed role '{}'", role_arn))]
//        CredentialsMissing { role_arn: String },
//    }
//}
//type Result<T> = std::result::Result<T, error::Error>;

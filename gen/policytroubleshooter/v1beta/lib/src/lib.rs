#![doc = "# Resources and Methods\n    * [iam](resources/iam/struct.IamActions.html)\n      * [*troubleshoot*](resources/iam/struct.TroubleshootRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaAccessTuple {
        #[doc = "Required. The full resource name that identifies the resource. For example,\n`//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.\n\nFor examples of full resource names for Google Cloud services, see\nhttps://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "Required. The IAM permission to check for the specified member and resource.\n\nFor a complete list of IAM permissions, see\nhttps://cloud.google.com/iam/help/permissions/reference.\n\nFor a complete list of predefined IAM roles and the permissions in each\nrole, see https://cloud.google.com/iam/help/roles/reference."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "Required. The member, or principal, whose access you want to check, in the form of\nthe email address that represents that member. For example,\n`alice@example.com` or\n`my-service-account@my-project.iam.gserviceaccount.com`.\n\nThe member must be a Google Account or a service account. Other types of\nmembers are not supported."]
        #[serde(
            rename = "principal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPolicytroubleshooterV1BetaAccessTuple {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPolicytroubleshooterV1BetaAccessTuple {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaBindingExplanation { # [ doc = "Indicates whether *this binding* provides the specified permission to the\nspecified member for the specified resource.\n\nThis field does *not* indicate whether the member actually has the\npermission for the resource. There might be another binding that overrides\nthis binding. To determine whether the member actually has the permission,\nuse the `access` field in the\nTroubleshootIamPolicyResponse." ] # [ serde ( rename = "access" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub access : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess > , # [ doc = "A condition expression that prevents access unless the expression evaluates\nto `true`.\n\nTo learn about IAM Conditions, see\nhttp://cloud.google.com/iam/help/conditions/overview." ] # [ serde ( rename = "condition" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub condition : :: std :: option :: Option < crate :: schemas :: GoogleTypeExpr > , # [ doc = "Indicates whether each member in the binding includes the member specified\nin the request, either directly or indirectly. Each key identifies a member\nin the binding, and each value indicates whether the member in the binding\nincludes the member in the request.\n\nFor example, suppose that a binding includes the following members:\n\n* `user:alice@example.com`\n* `group:product-eng@example.com`\n\nYou want to troubleshoot access for `user:bob@example.com`. This user is a\nmember of the group `group:product-eng@example.com`.\n\nFor the first member in the binding, the key is `user:alice@example.com`,\nand the `membership` field in the value is set to\n`MEMBERSHIP_NOT_INCLUDED`.\n\nFor the second member in the binding, the key is\n`group:product-eng@example.com`, and the `membership` field in the value is\nset to `MEMBERSHIP_INCLUDED`." ] # [ serde ( rename = "memberships" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub memberships : :: std :: option :: Option < :: std :: collections :: BTreeMap < String , crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembership > > , # [ doc = "The relevance of this binding to the overall determination for the entire\npolicy." ] # [ serde ( rename = "relevance" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance > , # [ doc = "The role that this binding grants. For example,\n`roles/compute.serviceAgent`.\n\nFor a complete list of predefined IAM roles, as well as the permissions in\neach role, see https://cloud.google.com/iam/help/roles/reference." ] # [ serde ( rename = "role" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub role : :: std :: option :: Option < String > , # [ doc = "Indicates whether the role granted by this binding contains the specified\npermission." ] # [ serde ( rename = "rolePermission" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub role_permission : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission > , # [ doc = "The relevance of the permission's existence, or nonexistence, in the role\nto the overall determination for the entire policy." ] # [ serde ( rename = "rolePermissionRelevance" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub role_permission_relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        #[doc = "Reserved for future use."]
        AccessStateUnspecified,
        #[doc = "The member has the permission."]
        Granted,
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[doc = "The member has the permission only if a condition expression evaluates to\n`true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that\nPolicy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: AccessStateUnspecified => "ACCESS_STATE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: Granted => "GRANTED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: NotGranted => "NOT_GRANTED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownConditional => "UNKNOWN_CONDITIONAL" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownInfoDenied => "UNKNOWN_INFO_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess, ()>
        {
            Ok ( match s { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownInfoDenied , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess :: UnknownInfoDenied , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAccess
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance {
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point\nis likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point\nis unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance,
            (),
        > {
            Ok ( match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: Normal , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance :: Normal , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRelevance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission {
        #[doc = "The permission is included in the role."]
        RolePermissionIncluded,
        #[doc = "The permission is not included in the role."]
        RolePermissionNotIncluded,
        #[doc = "The sender of the request is not allowed to access the binding."]
        RolePermissionUnknownInfoDenied,
        #[doc = "Reserved for future use."]
        RolePermissionUnspecified,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionIncluded => "ROLE_PERMISSION_INCLUDED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionNotIncluded => "ROLE_PERMISSION_NOT_INCLUDED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnknownInfoDenied => "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnspecified => "ROLE_PERMISSION_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission,
            (),
        > {
            Ok ( match s { "ROLE_PERMISSION_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionIncluded , "ROLE_PERMISSION_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionNotIncluded , "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnknownInfoDenied , "ROLE_PERMISSION_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ROLE_PERMISSION_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionIncluded , "ROLE_PERMISSION_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionNotIncluded , "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnknownInfoDenied , "ROLE_PERMISSION_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission :: RolePermissionUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermission
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance {
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point\nis likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point\nis unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance,
            (),
        > {
            Ok ( match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: Normal , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance :: Normal , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationRolePermissionRelevance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembership { # [ doc = "Indicates whether the binding includes the member." ] # [ serde ( rename = "membership" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub membership : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership > , # [ doc = "The relevance of the member's status to the overall determination for the\nbinding." ] # [ serde ( rename = "relevance" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembership
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembership
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership {
        #[doc = "The binding includes the member. The member can be included directly\nor indirectly. For example:\n\n* A member is included directly if that member is listed in the binding.\n* A member is included indirectly if that member is in a Google group or\n  G Suite domain that is listed in the binding."]
        MembershipIncluded,
        #[doc = "The binding does not include the member."]
        MembershipNotIncluded,
        #[doc = "The sender of the request is not allowed to access the binding."]
        MembershipUnknownInfoDenied,
        #[doc = "The member is an unsupported type. Only Google Accounts and service\naccounts are supported."]
        MembershipUnknownUnsupported,
        #[doc = "Reserved for future use."]
        MembershipUnspecified,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipIncluded => "MEMBERSHIP_INCLUDED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded => "MEMBERSHIP_NOT_INCLUDED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied => "MEMBERSHIP_UNKNOWN_INFO_DENIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported => "MEMBERSHIP_UNKNOWN_UNSUPPORTED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified => "MEMBERSHIP_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership,
            (),
        > {
            Ok ( match s { "MEMBERSHIP_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipIncluded , "MEMBERSHIP_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded , "MEMBERSHIP_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied , "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported , "MEMBERSHIP_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "MEMBERSHIP_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipIncluded , "MEMBERSHIP_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded , "MEMBERSHIP_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied , "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported , "MEMBERSHIP_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipMembership
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance {
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point\nis likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point\nis unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance,
            (),
        > {
            Ok ( match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: Normal , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance :: Normal , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaBindingExplanationAnnotatedMembershipRelevance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaExplainedPolicy {
        #[doc = "Indicates whether *this policy* provides the specified permission to the\nspecified member for the specified resource.\n\nThis field does *not* indicate whether the member actually has the\npermission for the resource. There might be another policy that overrides\nthis policy. To determine whether the member actually has the permission,\nuse the `access` field in the\nTroubleshootIamPolicyResponse."]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access: ::std::option::Option<
            crate::schemas::GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess,
        >,
        #[doc = "Details about how each binding in the policy affects the member's ability,\nor inability, to use the permission for the resource.\n\nIf the sender of the request does not have access to the policy, this field\nis omitted."]
        #[serde(
            rename = "bindingExplanations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binding_explanations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPolicytroubleshooterV1BetaBindingExplanation>,
        >,
        #[doc = "The full resource name that identifies the resource. For example,\n`//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.\n\nIf the sender of the request does not have access to the policy, this field\nis omitted.\n\nFor examples of full resource names for Google Cloud services, see\nhttps://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "The IAM policy attached to the resource.\n\nIf the sender of the request does not have access to the policy, this field\nis empty."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::GoogleIamV1Policy>,
        #[doc = "The relevance of this policy to the overall determination in the\nTroubleshootIamPolicyResponse.\n\nIf the sender of the request does not have access to the policy, this field\nis omitted."]
        #[serde(
            rename = "relevance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relevance: ::std::option::Option<
            crate::schemas::GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicy
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        #[doc = "Reserved for future use."]
        AccessStateUnspecified,
        #[doc = "The member has the permission."]
        Granted,
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[doc = "The member has the permission only if a condition expression evaluates to\n`true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that\nPolicy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: AccessStateUnspecified => "ACCESS_STATE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: Granted => "GRANTED" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: NotGranted => "NOT_GRANTED" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownConditional => "UNKNOWN_CONDITIONAL" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownInfoDenied => "UNKNOWN_INFO_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess, ()>
        {
            Ok ( match s { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownInfoDenied , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess :: UnknownInfoDenied , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyAccess
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        #[doc = "Reserved for future use."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point\nis likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point\nis unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance, ()>
        {
            Ok ( match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: Normal , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance :: Normal , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaExplainedPolicyRelevance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyRequest {
        #[doc = "The information to use for checking whether a member has a permission for a\nresource."]
        #[serde(
            rename = "accessTuple",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_tuple:
            ::std::option::Option<crate::schemas::GoogleCloudPolicytroubleshooterV1BetaAccessTuple>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponse { # [ doc = "Indicates whether the member has the specified permission for the specified\nresource, based on evaluating all of the applicable policies." ] # [ serde ( rename = "access" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub access : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess > , # [ doc = "List of IAM policies that were evaluated to check the member's permissions,\nwith annotations to indicate how each policy contributed to the final\nresult.\n\nThe list of policies can include the policy for the resource itself. It can\nalso include policies that are inherited from higher levels of the resource\nhierarchy, including the organization, the folder, and the project.\n\nTo learn more about the resource hierarchy, see\nhttps://cloud.google.com/iam/help/resource-hierarchy." ] # [ serde ( rename = "explainedPolicies" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub explained_policies : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaExplainedPolicy > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess {
        #[doc = "Reserved for future use."]
        AccessStateUnspecified,
        #[doc = "The member has the permission."]
        Granted,
        #[doc = "The member does not have the permission."]
        NotGranted,
        #[doc = "The member has the permission only if a condition expression evaluates to\n`true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that\nPolicy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: AccessStateUnspecified => "ACCESS_STATE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: Granted => "GRANTED" , GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: NotGranted => "NOT_GRANTED" , GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownConditional => "UNKNOWN_CONDITIONAL" , GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownInfoDenied => "UNKNOWN_INFO_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess,
            (),
        > {
            Ok ( match s { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownInfoDenied , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess :: UnknownInfoDenied , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponseAccess
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs:
            ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging.\nFor example, `storage.googleapis.com`, `cloudsql.googleapis.com`.\n`allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of\npermission.\nFollows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::GoogleIamV1AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIamV1AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl GoogleIamV1AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIamV1AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                GoogleIamV1AuditLogConfigLogType::DataRead => "DATA_READ",
                GoogleIamV1AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIamV1AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIamV1AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleIamV1AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIamV1AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIamV1AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIamV1AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfigLogType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Binding {
        #[doc = "The condition that is associated with this binding.\n\nIf the condition evaluates to `true`, then this binding applies to the\ncurrent request.\n\nIf the condition evaluates to `false`, then this binding does not apply to\nthe current request. However, a different role binding might grant the same\nrole to one or more of the members in this binding.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM\ndocumentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::GoogleTypeExpr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n  on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n  who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n  account. For example, `alice@example.com` .\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n  account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n  For example, `admins@example.com`.\n\n* `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a user that has been recently deleted. For\n  example, `alice@example.com?uid=123456789012345678901`. If the user is\n  recovered, this value reverts to `user:{emailid}` and the recovered user\n  retains the role in the binding.\n\n* `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus\n  unique identifier) representing a service account that has been recently\n  deleted. For example,\n  `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`.\n  If the service account is undeleted, this value reverts to\n  `serviceAccount:{emailid}` and the undeleted service account retains the\n  role in the binding.\n\n* `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a Google group that has been recently\n  deleted. For example, `admins@example.com?uid=123456789012345678901`. If\n  the group is recovered, this value reverts to `group:{emailid}` and the\n  recovered group retains the role in the binding.\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n  users of that domain. For example, `google.com` or `example.com`."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Binding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a\n`condition` that determines how and when the `bindings` are applied. Each\nof the `bindings` must contain at least one member."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::GoogleIamV1Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a policy from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform policy updates in order to avoid race\nconditions: An `etag` is returned in the response to `getIamPolicy`, and\nsystems are expected to put that etag in the request to `setIamPolicy` to\nensure that their change will be applied to the same version of the policy.\n\n**Important:** If you use IAM Conditions, you must include the `etag` field\nwhenever you call `setIamPolicy`. If you omit this field, then IAM allows\nyou to overwrite a version `3` policy with a version `1` policy, and all of\nthe conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy.\n\nValid values are `0`, `1`, and `3`. Requests that specify an invalid value\nare rejected.\n\nAny operation that affects conditional role bindings must specify version\n`3`. This requirement applies to the following operations:\n\n* Getting a policy that includes a conditional role binding\n* Adding a conditional role binding to a policy\n* Changing a conditional role binding in a policy\n* Removing any role binding, with or without a condition, from a policy\n  that includes conditions\n\n**Important:** If you use IAM Conditions, you must include the `etag` field\nwhenever you call `setIamPolicy`. If you omit this field, then IAM allows\nyou to overwrite a version `3` policy with a version `1` policy, and all of\nthe conditions in the version `3` policy are lost.\n\nIf a policy does not include any conditions, operations on that policy may\nspecify any valid version or leave the field unset.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Policy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleTypeExpr {
        #[doc = "Optional. Description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language\nsyntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeExpr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeExpr {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the iam resource"]
    pub fn iam(&self) -> crate::resources::iam::IamActions {
        crate::resources::iam::IamActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod iam {
        pub mod params {}
        pub struct IamActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> IamActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Checks whether a member has a specific permission for a specific resource,\nand explains why the member does or does not have that permission."]
            pub fn troubleshoot(
                &self,
                request : crate :: schemas :: GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyRequest,
            ) -> TroubleshootRequestBuilder {
                TroubleshootRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [IamActions::troubleshoot()](struct.IamActions.html#method.troubleshoot)"]
        #[derive(Debug, Clone)]
        pub struct TroubleshootRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request:
                crate::schemas::GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyRequest,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> TroubleshootRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudPolicytroubleshooterV1BetaTroubleshootIamPolicyResponse,
                crate::Error,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://policytroubleshooter.googleapis.com/".to_owned();
                output.push_str("v1beta/iam:troubleshoot");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

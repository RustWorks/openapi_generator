#![allow(clippy::clone_on_copy)]

{{#each paths}}
    {{~>operation_types get noBody=true}}
    {{~>operation_types head noBody=true}}
    {{~>operation_types post}}
    {{~>operation_types put}}
    {{~>operation_types delete}}
    {{~>operation_types options}}
    {{~>operation_types trace}}
    {{~>operation_types patch}}
{{~/each}}
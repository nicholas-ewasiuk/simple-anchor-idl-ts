### simple-anchor-idl-ts

A tool for generating a TS file from a lone Anchor IDL JSON file.

Creates a type and an object for the IDL.
Lower camel cases the account names.

Usually this TS file is generated when building an Anchor repo.
https://github.com/coral-xyz/anchor/blob/master/cli/src/rust_template.rs#L222

Sometimes you just have another project's JSON file and want a TS file for your client.

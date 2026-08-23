{{ config(materialized="table") }} select id, name, email from {{ ref("users") }}

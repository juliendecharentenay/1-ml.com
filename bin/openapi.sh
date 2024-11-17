#!/bin/bash

grep "\/\/\[openapi\]" \
  lambdas/oneml/src/api.rs \
  lambdas/oneml/src/api/authenticated_request.rs \
  lambdas/oneml/src/api/authenticated_request/me_get.rs \
  lambdas/oneml/src/api/authenticated_request/me_patch.rs \
  lambdas/oneml/src/api/authenticated_request/email_get.rs \
  lambdas/oneml/src/api/authenticated_request/email_patch.rs \
  lambdas/oneml/src/api/unauthenticated_request.rs \
| sed -e 's/^.*\[openapi] //' | sed -e 's/^.*\[openapi]//' > openapi.yaml

prism mock --port 8010 openapi.yaml


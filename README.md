# 1-ml.com
This repository presents the implementation of a mail masking service 
using AWS Simple Email Service (SES). The implementation is discussed
in the following story: https://medium.com/better-programming/implementing-an-email-masking-proof-of-concept-using-aws-simple-email-service-ses-and-aws-rust-c6aa34f1e1df

This implementation powers https://www.1-ml.com.

The implementation is divided across the following directories:

* `vue3`: a VueJS based website;

# Installation and Usage

## Pre-requisites:

* [Node.js](https://nodejs.org/en/) - for compilation of website;

## Website:

##
API mocking for development purposes:

```
# Pre-requisite
npm install -g @stoplight/prism-cli

# Run a mock API server
#  - Creates an openapi.yaml file
#  - start mock server (using prism) on port 8010
./openapi.sh
```



# rust-license

**rust-license** is a tool inspired by [go-license](https://github.com/palantir/go-license).<br>
This tool ensures that a license header is applied to a list of files.

# Install

> cargo install rust-license

# Usage

Create a yaml config file `rust-license.yaml` containing the headers you want to apply in your project

```
headers: |
  // Copyright (c) 2024 najeal, All rights reserved.
  // See the file LICENSE for licensing terms.
```

## Apply headers
using the tool you can apply this license to your files (with **--apply** flag):<br>
`rust-license license-header --config rust-license.yaml --apply your-first-file.txt your-second-file.rust`

## Check headers
you can check the license is in your files (with **--check** flag), the tool will print the paths of files not containing the header:<br>
`rust-license license-header --config rust-license.yaml --check your-first-file.txt your-second-file.rust`

## Remove headers
you can delete the license from your files (with **--remove** flag):<br>
`rust-license license-header --config rust-license.yaml --remove your-first-file.txt your-second-file.rust`
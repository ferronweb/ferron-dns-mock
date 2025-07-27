# Ferron mock DNS provider

This is a mock DNS provider for Ferron, demonstrating how to create a custom DNS provider for ACME with Ferron.

## Compiling Ferron with this provider

To compile Ferron with this provider, first clone the Ferron repository:

```bash
git clone https://github.com/ferronweb/ferron.git -b develop-2.x
cd ferron
```

Then, copy the `ferron-build.yaml` file to `ferron-build-override.yaml`:

```bash
cp ferron-build.yaml ferron-build-override.yaml
```

After that, add the following line to the `ferron-build-override.yaml` file:

```yaml
dns:
  # Other modules...
  - git: https://github.com/ferronweb/ferron-dns-mock.git
    crate: ferron-dns-mock
    loader: MockDnsProvider
```

After modifying the `ferron-build-override.yaml` file, you can compile Ferron with this module by running the following command:

```bash
make build
```

You can then package it in a ZIP archive using the following command:

```bash
make package
```

The ZIP archive will be created in the `dist` directory, and can be installed using Ferron installer.

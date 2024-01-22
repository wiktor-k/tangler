<!--
SPDX-FileCopyrightText: 2021-2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `ssh-openpgp-auth`

This tool provides client-side functionality to transparently verify the identity of remote SSH hosts, based on trust chains rooted in the user's OpenPGP configuration.

Concretely, this tool fetches OpenPGP certificates for remote SSH hosts before opening SSH connections. The host certificate is verified based on OpenPGP trust chains, starting from the user's configured trust roots. If there is a valid trust chain, this tool adds the remote host's SSH public key to the local OpenSSH "known host" configuration.

To gracefully handle host key life cycle events, the remote host's OpenPGP certificate is automatically refreshed when it expires.

All OpenPGP certificates are stored locally in the standard [CertD](https://datatracker.ietf.org/doc/draft-nwjw-openpgp-cert-d/) directory (e.g. on Linux the default path is `.local/share/pgp.cert.d`, see [3.8. Platform-specific conventions](https://www.ietf.org/archive/id/draft-nwjw-openpgp-cert-d-00.html#section-3.8)).

## Installation

This tool can be used either on a per-remote-host basis, or globally, by editing the `.ssh/config` file:

```
Host example.com
	KnownHostsCommand /usr/bin/ssh-openpgp-auth authenticate %H
```

## Verification flags

By default, this tool fetches missing certificates and does basic integrity checks on the remote host's certificate. It is possible to enable stricter checks by appending additional flags. Additional verification flags cause the tool to perform stricter checks.

### Web of Trust verification (`--verify-wot`)

The following example illustrates defining trust roots in the user's OpenPGP certificate store, to perform host certificate verification using the Web of Trust.

Note that if the user already has a Web of Trust setup (e.g., to rely on their organization's OpenPGP CA instance), these trust roots are leveraged automatically. This tool will then "just work", and rely on chains from trust roots to remote host certificates. Remote host certificates are automatically fetched over the network (using the WKD protocol), and trust calculations happen locally whenever the tool runs.

### DNS/Keyoxide proof verification (`--verify-dns-proof`)

This validation requires that the key fingerprint is present in the DNS zone of a host:

```
$ dig +short TXT metacode.biz
"openpgp4fpr:198c722a4bac336e9daaae44579d01b3abe1540e"
"openpgp4fpr:653909a2f0e37c106f5faf546c8857e0d8e8f074"
```

The exact format will be specified in the future (see [issue #25](https://codeberg.org/wiktor/ssh-openpgp-auth/issues/25)).

## Usage example

Let's do an example run of using this tool in an isolated environment. To set up a test environment, we configure a temporary directory to use as OpenPGP certificate store:

```sh
export SQ_CERT_STORE=$(mktemp -d)
export PGP_CERT_D=$SQ_CERT_STORE
```

Then we import the sample host certificate (which has the fingerprint `D9E95D7F42E87610676C40B47E8432836DA1625E`) into our temporary local certificate store, and directly configure it as a local trust root:

```sh
sq import < fixtures/example.asc

sq link add --all D9E95D7F42E87610676C40B47E8432836DA1625E
```

After defining the certificate as a trust root, the "known hosts" configuration contains a single authentication key in SSH format:

```sh
KNOWN_HOSTS=$(ssh-openpgp-auth authenticate --verify-wot example.com)

[ "$KNOWN_HOSTS" = "example.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIUDdSi3z0/ePRqteV3Gk5MRZ7ZKenxcqDatCZDzimpLSTA== D9E95D7F42E87610676C40B47E8432836DA1625E/F5CEDEED08E9EA536034F5823475162385DF08AF" ]
```

Next, we retract the link again, which means that we don't rely on this certificate as a trust root anymore.

Note that because the resolution of timestamps on OpenPGP signatures is limited to full seconds, we wait for two seconds to make sure the changed trust root configuration is in effect:

```sh
sleep 2
sq --force link retract D9E95D7F42E87610676C40B47E8432836DA1625E '<ssh-openpgp-auth@example.com>'
sleep 2
```

After unsetting this trust root, the SSH "known hosts" configuration is empty again, because the Web of Trust verification does not yield any valid SSH host certificates:

```sh
KNOWN_HOSTS=$(ssh-openpgp-auth authenticate --verify-wot example.com)

[ "$KNOWN_HOSTS" = "" ]
```

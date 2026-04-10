export SQ_CERT_STORE=$(mktemp -d)
export PGP_CERT_D=$SQ_CERT_STORE

sq import < fixtures/example.asc

sq link add --all D9E95D7F42E87610676C40B47E8432836DA1625E

KNOWN_HOSTS=$(ssh-openpgp-auth authenticate --verify-wot example.com)

[ "$KNOWN_HOSTS" = "example.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIUDdSi3z0/ePRqteV3Gk5MRZ7ZKenxcqDatCZDzimpLSTA== D9E95D7F42E87610676C40B47E8432836DA1625E/F5CEDEED08E9EA536034F5823475162385DF08AF" ]

sleep 2
sq --force link retract D9E95D7F42E87610676C40B47E8432836DA1625E '<ssh-openpgp-auth@example.com>'
sleep 2

KNOWN_HOSTS=$(ssh-openpgp-auth authenticate --verify-wot example.com)

[ "$KNOWN_HOSTS" = "" ]


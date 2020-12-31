all: wasi_ephemeral_crypto.txt wasi_ephemeral_crypto.json wasi_ephemeral_crypto.md proposal_common.md proposal_asymmetric_common.md proposal_symmetric.md proposal_signatures.md proposal_kx.md proposal_external_secrets.md

wasi_ephemeral_crypto.txt: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx wasi_ephemeral_crypto.witx proposal_kx.witx proposal_external_secrets.witx
	witx-overview-docgen -o $@ wasi_ephemeral_crypto.witx

wasi_ephemeral_crypto.json: wasi_ephemeral_crypto.txt
	witx-lucet-bindings-gen -o $@ wasi_ephemeral_crypto.witx

wasi_ephemeral_crypto.md: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx wasi_ephemeral_crypto.witx proposal_kx.md proposal_external_secrets.md
	witx-docgen -o $@ wasi_ephemeral_crypto.witx

proposal_common.md: proposal_common.witx
	witx-docgen -o $@ proposal_common.witx

proposal_asymmetric_common.md: proposal_common.witx proposal_asymmetric_common.witx
	witx-docgen -o $@ proposal_asymmetric_common.witx

proposal_symmetric.md: proposal_common.witx proposal_symmetric.witx
	witx-docgen -o $@ proposal_symmetric.witx

proposal_signatures.md: proposal_common.witx proposal_asymmetric_common.witx proposal_signatures.witx
	witx-docgen -o $@ proposal_signatures.witx

proposal_kx.md: proposal_common.witx proposal_asymmetric_common.witx proposal_kx.witx
	witx-docgen -o $@ proposal_kx.witx

proposal_external_secrets.md: proposal_common.witx proposal_external_secrets.witx
	witx-docgen -o $@ proposal_external_secrets.witx

clean:
	-rm -f *.md *.txt

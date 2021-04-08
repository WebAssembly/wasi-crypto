all: wasi_ephemeral_crypto.txt wasi_ephemeral_crypto.md proposal_common.md proposal_asymmetric_common.md proposal_symmetric.md proposal_signatures.md proposal_kx.md proposal_external_secrets.md

wasi_ephemeral_crypto.txt: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx proposal_kx.witx proposal_external_secrets.witx
	witx-codegen --output-type overview -o $@ proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx proposal_kx.witx proposal_external_secrets.witx

wasi_ephemeral_crypto.md: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx proposal_kx.witx proposal_external_secrets.witx
	witx-codegen --output-type markdown -o $@ proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx proposal_kx.witx proposal_external_secrets.witx

proposal_common.md: proposal_common.witx
	witx-codegen --output-type markdown -o $@ proposal_common.witx

proposal_asymmetric_common.md: proposal_common.witx proposal_asymmetric_common.witx
	witx-codegen --output-type markdown -o $@ proposal_asymmetric_common.witx

proposal_symmetric.md: proposal_common.witx proposal_symmetric.witx
	witx-codegen --output-type markdown -o $@ proposal_symmetric.witx

proposal_signatures.md: proposal_common.witx proposal_asymmetric_common.witx proposal_signatures.witx
	witx-codegen --output-type markdown -o $@ proposal_signatures.witx

proposal_kx.md: proposal_common.witx proposal_asymmetric_common.witx proposal_kx.witx
	witx-codegen --output-type markdown -o $@ proposal_kx.witx

proposal_external_secrets.md: proposal_common.witx proposal_external_secrets.witx
	witx-codegen --output-type markdown -o $@ proposal_external_secrets.witx

clean:
	-rm -f *.md *.txt

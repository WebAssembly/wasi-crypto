all: wasi_ephemeral_crypto.md proposal_symmetric.md proposal_signatures.md

wasi_ephemeral_crypto.md: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx
	witx-docgen -o $@ proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx proposal_signatures.witx

proposal_symmetric.md: proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx
	witx-docgen -o $@ proposal_common.witx proposal_asymmetric_common.witx proposal_symmetric.witx

proposal_signatures.md: proposal_common.witx proposal_asymmetric_common.witx proposal_signatures.witx
	witx-docgen -o $@ proposal_common.witx proposal_asymmetric_common.witx proposal_signatures.witx

clean:
	-rm -f *.md

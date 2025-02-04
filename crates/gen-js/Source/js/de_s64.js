function deserializeS64(de) {
	const n = de_varint_big(de, 64);

	return (
		((n >> 1n) & 0xffffffffffffffffn) ^ -(n & 0b1n & 0xffffffffffffffffn)
	);
}

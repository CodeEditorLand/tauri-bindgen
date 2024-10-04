function deserializeS16(de) {
	const n = de_varint(de, 16);

	return Number(((n >> 1) & 0xffff) ^ -(n & 0b1 & 0xffff));
}

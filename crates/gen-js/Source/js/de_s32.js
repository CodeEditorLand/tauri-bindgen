function deserializeS32(de) {
	const n = de_varint(de, 32);

	return Number(((n >> 1) & 0xffffffff) ^ -(n & 0b1 & 0xffffffff));
}

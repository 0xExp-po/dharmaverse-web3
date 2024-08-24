'use client';

import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import { PublicKey, Connection } from "@solana/web3.js";


export const getMetadata = async ({ address }: { address: PublicKey }) => {
    const connection = new Connection('https://api.devnet.solana.com');

    const [metadataPDA, _] = await PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
            address.toBuffer(),
        ],
        new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
    );

    const mintAccInfo = await connection.getAccountInfo(metadataPDA);

    if (!mintAccInfo) {
        throw new Error('Metadata account not found');
      }
    const metadata = Metadata.deserialize(mintAccInfo?.data!)[0];

    return metadata
};

import { AnchorProvider, web3} from "@coral-xyz/anchor";
import {
    createInitializeMintInstruction,
    MintLayout,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

export const createMint = async (
    provider: AnchorProvider,
    decimals = 0
): Promise<web3.PublicKey> => {
    //token key pair
    const tokenMint = new web3.Keypair();
    //calculate rent
    const lamportsForMint = await provider.connection.getMinimumBalanceForRentExemption(
        MintLayout.span
    );
    /////
    await provider.sendAndConfirm(
        new web3.Transaction()
        .add(
            //create mint account
            web3.SystemProgram.createAccount({
                programId: TOKEN_PROGRAM_ID,            //program id
                space: MintLayout.span,                 //space
                fromPubkey: provider.wallet.publicKey,  //payer
                newAccountPubkey: tokenMint.publicKey,  //token addreass
                lamports: lamportsForMint,              //rent
            })
        )
        .add(
            createInitializeMintInstruction(
                tokenMint.publicKey,        // token address
                decimals,                    // decimals     
                provider.wallet.publicKey, // mint authority
                provider.wallet.publicKey   // freeze authority
            )
        ),
        [tokenMint]//signer
    );
    //return new Token address
    return tokenMint.publicKey;
};
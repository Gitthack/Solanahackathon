import { BitgetWallet } from '@bitget-wallet/sdk';
import { Connection, PublicKey, Transaction, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createTransferInstruction } from '@solana/spl-token';

// USDC mint on Solana devnet
const USDC_MINT_DEVNET = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');

interface PaymentRequest {
  recipient: string;
  amount: number; // USDC amount
  taskId: string;
  memo?: string;
}

interface WalletBalance {
  sol: number;
  usdc: number;
}

/**
 * Bitget Wallet integration for Kings Agent Guild
 * Handles wallet connection, payments, and transaction signing
 */
export class BitgetWalletIntegration {
  private wallet: BitgetWallet | null = null;
  private connection: Connection;
  private publicKey: PublicKey | null = null;

  constructor(rpcUrl: string = 'https://api.devnet.solana.com') {
    this.connection = new Connection(rpcUrl, 'confirmed');
  }

  /**
   * Connect to Bitget Wallet
   */
  async connect(): Promise<string | null> {
    try {
      this.wallet = new BitgetWallet({
        network: 'solana-devnet',
      });

      const connected = await this.wallet.connect();
      if (connected) {
        this.publicKey = new PublicKey(this.wallet.publicKey!);
        console.log('Connected to Bitget Wallet:', this.publicKey.toBase58());
        return this.publicKey.toBase58();
      }
      return null;
    } catch (error) {
      console.error('Failed to connect Bitget Wallet:', error);
      throw error;
    }
  }

  /**
   * Disconnect wallet
   */
  async disconnect(): Promise<void> {
    if (this.wallet) {
      await this.wallet.disconnect();
      this.wallet = null;
      this.publicKey = null;
    }
  }

  /**
   * Get wallet balance (SOL and USDC)
   */
  async getBalance(): Promise<WalletBalance> {
    if (!this.publicKey) {
      throw new Error('Wallet not connected');
    }

    try {
      // Get SOL balance
      const solBalance = await this.connection.getBalance(this.publicKey);

      // Get USDC balance
      const usdcAccount = await getAssociatedTokenAddress(
        USDC_MINT_DEVNET,
        this.publicKey
      );

      let usdcBalance = 0;
      try {
        const accountInfo = await this.connection.getTokenAccountBalance(usdcAccount);
        usdcBalance = accountInfo.value.uiAmount || 0;
      } catch (e) {
        // USDC account doesn't exist yet
        usdcBalance = 0;
      }

      return {
        sol: solBalance / LAMPORTS_PER_SOL,
        usdc: usdcBalance,
      };
    } catch (error) {
      console.error('Failed to get balance:', error);
      throw error;
    }
  }

  /**
   * Execute USDC payment using x402 protocol
   * This is the core function for Agent-to-Agent payments
   */
  async executePayment(request: PaymentRequest): Promise<string> {
    if (!this.wallet || !this.publicKey) {
      throw new Error('Wallet not connected');
    }

    try {
      const recipient = new PublicKey(request.recipient);
      const amount = request.amount * 1_000_000; // USDC has 6 decimals

      // Get sender's USDC token account
      const senderTokenAccount = await getAssociatedTokenAddress(
        USDC_MINT_DEVNET,
        this.publicKey
      );

      // Get recipient's USDC token account
      const recipientTokenAccount = await getAssociatedTokenAddress(
        USDC_MINT_DEVNET,
        recipient
      );

      // Create transfer instruction
      const transferInstruction = createTransferInstruction(
        senderTokenAccount,
        recipientTokenAccount,
        this.publicKey,
        amount,
        [],
        TOKEN_PROGRAM_ID
      );

      // Create transaction
      const transaction = new Transaction().add(transferInstruction);

      // Add memo with task ID (for tracking)
      if (request.memo || request.taskId) {
        const memoText = request.memo || `Task: ${request.taskId}`;
        transaction.add(
          SystemProgram.transfer({
            fromPubkey: this.publicKey,
            toPubkey: this.publicKey,
            lamports: 0,
          })
        );
        // Note: In production, use proper memo program
      }

      // Get recent blockhash
      const { blockhash } = await this.connection.getLatestBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = this.publicKey;

      // Sign and send transaction
      const signed = await this.wallet.signTransaction(transaction);
      const signature = await this.connection.sendRawTransaction(signed.serialize());

      // Wait for confirmation
      await this.connection.confirmTransaction(signature, 'confirmed');

      console.log('Payment executed:', signature);
      console.log('Task ID:', request.taskId);
      console.log('Amount:', request.amount, 'USDC');
      console.log('Recipient:', request.recipient);

      return signature;
    } catch (error) {
      console.error('Payment failed:', error);
      throw error;
    }
  }

  /**
   * Request testnet USDC (for development)
   */
  async requestTestUSDC(): Promise<string | null> {
    if (!this.publicKey) {
      throw new Error('Wallet not connected');
    }

    try {
      // This would call a faucet API in production
      // For now, return a placeholder
      console.log('Requesting test USDC for:', this.publicKey.toBase58());
      return 'faucet_request_sent';
    } catch (error) {
      console.error('Failed to request test USDC:', error);
      return null;
    }
  }

  /**
   * Get connected wallet address
   */
  getAddress(): string | null {
    return this.publicKey?.toBase58() || null;
  }

  /**
   * Check if wallet is connected
   */
  isConnected(): boolean {
    return this.wallet?.isConnected || false;
  }
}

/**
 * Payment helper for escrow integration
 * Handles the x402 payment flow between employer and worker
 */
export class EscrowPaymentHelper {
  private walletIntegration: BitgetWalletIntegration;

  constructor(walletIntegration: BitgetWalletIntegration) {
    this.walletIntegration = walletIntegration;
  }

  /**
   * Lock payment in escrow (employer side)
   */
  async lockPayment(
    escrowAddress: string,
    amount: number,
    taskId: string
  ): Promise<string> {
    // In production, this would interact with the escrow smart contract
    // For now, we simulate the lock by showing the intent
    console.log('Locking payment in escrow:', {
      escrow: escrowAddress,
      amount,
      taskId,
    });

    // Transfer to escrow address
    return this.walletIntegration.executePayment({
      recipient: escrowAddress,
      amount,
      taskId,
      memo: `ESCROW_LOCK:${taskId}`,
    });
  }

  /**
   * Release payment from escrow to worker
   */
  async releasePayment(
    workerAddress: string,
    amount: number,
    taskId: string
  ): Promise<string> {
    console.log('Releasing payment to worker:', {
      worker: workerAddress,
      amount,
      taskId,
    });

    return this.walletIntegration.executePayment({
      recipient: workerAddress,
      amount,
      taskId,
      memo: `ESCROW_RELEASE:${taskId}`,
    });
  }
}

// Export singleton instance
export const bitgetWallet = new BitgetWalletIntegration();

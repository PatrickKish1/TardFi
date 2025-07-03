import { getDefaultConfig } from '@rainbow-me/rainbowkit';
import { mainnet, sepolia } from 'wagmi/chains';

export const projectId = process.env.NEXT_PUBLIC_PROJECT_ID!;

export const config = getDefaultConfig({
  appName: 'TardFi',
  projectId,
  chains: [mainnet, sepolia],
  ssr: true,
}); 
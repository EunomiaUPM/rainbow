import { Inter } from 'next/font/google';
import { Provider } from '@/components/provider';
import './global.css';
import {BackgroundGrid} from "@/components/background-grid";

const inter = Inter({
  subsets: ['latin'],
});

export default function Layout({ children }: LayoutProps<'/'>) {
  return (
    <html lang="en" className={inter.className} suppressHydrationWarning>
      <body className="flex flex-col min-h-screen">
        <Provider>
            <BackgroundGrid />
            {children}</Provider>
      </body>
    </html>
  );
}

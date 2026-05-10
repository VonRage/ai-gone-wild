import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Linux Game Optimizer",
  description: "Native Linux game optimization daemon",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">
        {children}
      </body>
    </html>
  );
}

import type {Metadata} from "next";
import {Noto_Sans_JP} from "next/font/google";
import "./globals.css";
import {SideBar} from "@/components/SideBar";

const notoSansJP = Noto_Sans_JP({
	subsets: ["latin"],
});

export const metadata: Metadata = {
	title: "Create Next App",
	description: "Generated by create next app",
};

export default function RootLayout({
																		 children,
																	 }: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
		<body className={`${notoSansJP.className} w-screen h-screen flex flex-row overflow-hidden whitespace-nowrap`}>
		<SideBar className={"flex-grow-0 overflow-auto"}/>
		<div className={"h-screen flex-grow overflow-auto flex"}>
			{children}
		</div>
		</body>
		</html>
	);
}
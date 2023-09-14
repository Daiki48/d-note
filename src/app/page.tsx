import { css } from "../../styled-system/css";
// import { css } from "@pandacss/dev";
import Link from "next/link";

export default function Home() {
  return (
    <main className="">
      <h1 className={css({ color: "blue", fontSize: "xl" })}>Dnote</h1>
      <p>Github Actionsによる自動デプロイを実装</p>
      <p>Next.jsで実装</p>
      <div
        className={css({ fontSize: "2xl", fontWeight: "bold", color: "red" })}
      >
        Hello 🐼!
      </div>
      <Link href="/blog">Blogのページへ</Link>
    </main>
  );
}

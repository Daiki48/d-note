import Link from "next/link";

export default function Home() {
  return (
    <main className="">
      <h1>Dnote</h1>
      <p>Github Actionsによる自動デプロイを実装</p>
      <p>Next.jsで実装</p>
      <Link href="/blog">Blogのページへ</Link>
    </main>
  );
}

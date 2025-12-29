import type { ReactNode } from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Heading from "@theme/Heading";
import Head from "@docusaurus/Head";
import Translate, { translate } from "@docusaurus/Translate";
import useBaseUrl, { useBaseUrlUtils } from "@docusaurus/useBaseUrl";

import styles from "./index.module.css";

function HeroBanner() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className={styles.heroProjectTagline}>
          <img
            className={styles.heroLogo}
            src={useBaseUrl("/img/logo.png")}
            width="200"
            height="200"
          />
          <span
            className={styles.heroTitleTextHtml}
            // eslint-disable-next-line react/no-danger
            dangerouslySetInnerHTML={{
              __html: translate({
                id: "homepage.hero.title",
                message:
                  "Build <b>optimized</b> websites <b>quickly</b>, focus on your <b>content</b>",
                description:
                  "Home page hero title, can contain simple html tags",
              }),
            }}
          />
        </Heading>
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/intro"
          >
            Get Griffon for Linux (AppImage)
          </Link>
        </div>
        <span className={styles.indexCtasGitHubButtonWrapper}>
          <iframe
            className={styles.indexCtasGitHubButton}
            src="https://ghbtns.com/github-btn.html?user=GriffonAV&repo=GriffonAV&type=star&count=true&size=large"
            width={160}
            height={30}
            title="GitHub Stars"
          />
        </span>
      </div>
    </header>
  );
}

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return <HeroBanner />;
}

export default function Home(): ReactNode {
  const { siteConfig } = useDocusaurusContext();

  const title = `Griffon | Linux Antivirus`;
  const description = "Keep it safe. Simple and efficient.";
  const imageUrl = "https://griffon-av.vercel.app/img/griffon.png";

  return (
    <Layout title={title} description={description}>
      <Head>
        <meta property="og:title" content={title} />
        <meta property="og:description" content={description} />
        <meta property="og:image" content={imageUrl} />
        <meta property="og:url" content="https://griffon-av.vercel.app" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content={title} />
        <meta name="twitter:description" content={description} />
        <meta name="twitter:image" content={imageUrl} />
      </Head>
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}

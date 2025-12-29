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
import styles from "./styles.module.css";

function HeroBanner() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <div className={styles.hero} data-theme="dark">
      <div className={styles.heroInner}>
        <Heading as="h1" className={styles.heroProjectTagline}>
          <img
            alt={translate({ message: "Docusaurus with Keytar" })}
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
                  "A <b>Fast, modular, and secure</b> security engine for Linux, Writen in <b>Rust</b>",
                description:
                  "Home page hero title, can contain simple html tags",
              }),
            }}
          />
        </Heading>
        <div className={styles.indexCtas}>
          <Link className="button button--primary" to="/docs/intro">
            <Translate>Get Started</Translate>
          </Link>
          <Link className="button button--info" to="/docs/intro">
            <Translate>Try a Demo</Translate>
          </Link>
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
      </div>
    </div>
  );
}

function TopBannerForRelease() {
  // TODO We should be able to strongly type customFields
  //  Refactor to use a CustomFields interface + TS declaration merging
  const announcedVersion = useDocusaurusContext().siteConfig.customFields
    ?.announcedVersion as string;

  return (
    <div className={styles.topBanner}>
      <div className={styles.topBannerTitle}>
        {"🎉\xa0"}
        <Link
          to={`/blog/releases/${announcedVersion}`}
          className={styles.topBannerTitleText}
        >
          <Translate
            id="homepage.banner.launch.newVersion"
            values={{ newVersion: announcedVersion }}
          >
            {"Docusaurus\xa0{newVersion} is\xa0out!️"}
          </Translate>
        </Link>
        {"\xa0🥳"}
      </div>
    </div>
  );
}

function TopBanner() {
  // TODO We should be able to strongly type customFields
  //  Refactor to use a CustomFields interface + TS declaration merging
  const announcedVersion = useDocusaurusContext().siteConfig.customFields
    ?.announcedVersion as string;

  return (
    <div className={styles.topBanner}>
      <div className={styles.topBannerTitle}>
        {"\xa0🚧\xa0"}
        Work in Progress
        {"\xa0🚧\xa0"}
      </div>
    </div>
  );
}

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <main>
      <TopBanner />
      <HeroBanner />
    </main>
  );
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

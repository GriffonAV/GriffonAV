import type { ReactNode } from "react";
import clsx from "clsx";
import Heading from "@theme/Heading";
import styles from "./styles.module.css";

type FeatureItem = {
  title: string;
  // Svg: React.ComponentType<React.ComponentProps<"svg">>;
  Img: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  // {
  //   title: "Griffon",
  //   // Svg: require("@site/static/img/undraw_docusaurus_mountain.svg").default,
  //   Img: require("@site/static/img/rust.png").default,
  //   description: (
  //     <>
  //       Griffon is a school project aimed at developing an modular tools for
  //       linux security focusing on the analysis of malware and suspicious files.
  //       It is designed to be easy to use, with a simple and intuitive interface
  //       that allows users to quickly analyze files and identify potential
  //       threats.
  //     </>
  //   ),
  // },
  {
    title: "Powered by Rust",
    Img: require("@site/static/img/rust.png").default,
    description: (
      <>
        Griffon AV is built using Rust, a modern systems programming language
        known for its performance, safety, and concurrency. This ensures that
        our autonomous vehicle solutions are both reliable and efficient.
      </>
    ),
  },
  {
    title: "Research Oriented and Transparent Project",
    Img: require("@site/static/img/notion.png").default,
    description: (
      <>
        All our organization's research is open source and transparent. We are
        committed to making our research accessible to everyone. You can find
        our notion page{" "}
        <a href="https://blue-touch-18c.notion.site/Griffon-AV-1c6f05587c8380eb9fbeea36f549fd47?pvs=74">
          here
        </a>
      </>
    ),
  },
];

function Feature({ title, Img, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <img src={Img} className={styles.featureImg} alt={title} />{" "}
        {/* Use img tag */}
        {/* <Svg className={styles.featureSvg} role="img" /> */}
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}

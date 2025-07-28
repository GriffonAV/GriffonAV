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
    {
        title: "Easy to Use",
        // Svg: require("@site/static/img/undraw_docusaurus_mountain.svg").default,
        Img: require("@site/static/img/rust.png").default,
        description: (
            <>
                Docusaurus was designed from the ground up to be easily
                installed and used to get your website up and running quickly.
            </>
        ),
    },
    {
        title: "Focus on What Matters",
        Img: require("@site/static/img/rust.png").default,
        description: (
            <>
                Docusaurus lets you focus on your docs, and we&apos;ll do the
                chores. Go ahead and move your docs into the <code>docs</code>{" "}
                directory.
            </>
        ),
    },
    {
        title: "Powered by React",
        Img: require("@site/static/img/rust.png").default,
        description: (
            <>
                Extend or customize your website layout by reusing React.
                Docusaurus can be extended while reusing the same header and
                footer.
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

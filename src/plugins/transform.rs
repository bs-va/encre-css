/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("origin-center", "transform-origin: center;".to_string());
    selectors.register("origin-top", "transform-origin: top;".to_string());
    selectors.register(
        "origin-top-right",
        "transform-origin: top right;".to_string(),
    );
    selectors.register("origin-right", "transform-origin: right;".to_string());
    selectors.register(
        "origin-bottom-right",
        "transform-origin: bottom right;".to_string(),
    );
    selectors.register("origin-bottom", "transform-origin: bottom;".to_string());
    selectors.register(
        "origin-bottom-left",
        "transform-origin: bottom left;".to_string(),
    );
    selectors.register("origin-left", "transform-origin: left;".to_string());
    selectors.register("origin-top-left", "transform-origin: top left;".to_string());
    selectors.register(
        "scale-0",
        "--transform-scale-x: 0; --transform-scale-y: 0;".to_string(),
    );
    selectors.register(
        "scale-50",
        "--transform-scale-x: .5; --transform-scale-y: .5;".to_string(),
    );
    selectors.register(
        "scale-75",
        "--transform-scale-x: .75; --transform-scale-y: .75;".to_string(),
    );
    selectors.register(
        "scale-90",
        "--transform-scale-x: .9; --transform-scale-y: .9;".to_string(),
    );
    selectors.register(
        "scale-95",
        "--transform-scale-x: .95; --transform-scale-y: .95;".to_string(),
    );
    selectors.register(
        "scale-100",
        "--transform-scale-x: 1; --transform-scale-y: 1;".to_string(),
    );
    selectors.register(
        "scale-105",
        "--transform-scale-x: 1.05; --transform-scale-y: 1.05;".to_string(),
    );
    selectors.register(
        "scale-110",
        "--transform-scale-x: 1.1; --transform-scale-y: 1.1;".to_string(),
    );
    selectors.register(
        "scale-125",
        "--transform-scale-x: 1.25; --transform-scale-y: 1.25;".to_string(),
    );
    selectors.register(
        "scale-150",
        "--transform-scale-x: 1.5; --transform-scale-y: 1.5;".to_string(),
    );
    selectors.register("scale-x-0", "--transform-scale-x: 0;".to_string());
    selectors.register("scale-x-50", "--transform-scale-x: .5;".to_string());
    selectors.register("scale-x-75", "--transform-scale-x: .75;".to_string());
    selectors.register("scale-x-90", "--transform-scale-x: .9;".to_string());
    selectors.register("scale-x-95", "--transform-scale-x: .95;".to_string());
    selectors.register("scale-x-100", "--transform-scale-x: 1;".to_string());
    selectors.register("scale-x-105", "--transform-scale-x: 1.05;".to_string());
    selectors.register("scale-x-110", "--transform-scale-x: 1.1;".to_string());
    selectors.register("scale-x-125", "--transform-scale-x: 1.25;".to_string());
    selectors.register("scale-x-150", "--transform-scale-x: 1.5;".to_string());
    selectors.register("scale-y-0", "--transform-scale-y: 0;".to_string());
    selectors.register("scale-y-50", "--transform-scale-y: 0.5;".to_string());
    selectors.register("scale-y-75", "--transform-scale-y: 0.75;".to_string());
    selectors.register("scale-y-90", "--transform-scale-y: 0.9;".to_string());
    selectors.register("scale-y-95", "--transform-scale-y: 0.95;".to_string());
    selectors.register("scale-y-100", "--transform-scale-y: 1;".to_string());
    selectors.register("scale-y-105", "--transform-scale-y: 1.05;".to_string());
    selectors.register("scale-y-110", "--transform-scale-y: 1.1;".to_string());
    selectors.register("scale-y-125", "--transform-scale-y: 1.25;".to_string());
    selectors.register("scale-y-150", "--transform-scale-y: 1.5;".to_string());
    selectors.register("rotate-0", "--transform-rotate: 0;".to_string());
    selectors.register("rotate-1", "--transform-rotate: 1deg;".to_string());
    selectors.register("rotate-2", "--transform-rotate: 2deg;".to_string());
    selectors.register("rotate-3", "--transform-rotate: 3deg;".to_string());
    selectors.register("rotate-6", "--transform-rotate: 6deg;".to_string());
    selectors.register("rotate-12", "--transform-rotate: 12deg;".to_string());
    selectors.register("rotate-45", "--transform-rotate: 45deg;".to_string());
    selectors.register("rotate-90", "--transform-rotate: 90deg;".to_string());
    selectors.register("rotate-180", "--transform-rotate: 180deg;".to_string());
    selectors.register("-rotate-180", "--transform-rotate: -180deg;".to_string());
    selectors.register("-rotate-90", "--transform-rotate: -90deg;".to_string());
    selectors.register("-rotate-45", "--transform-rotate: -45deg;".to_string());
    selectors.register("-rotate-12", "--transform-rotate: -12deg;".to_string());
    selectors.register("-rotate-6", "--transform-rotate: -6deg;".to_string());
    selectors.register("-rotate-3", "--transform-rotate: -3deg;".to_string());
    selectors.register("-rotate-2", "--transform-rotate: -2deg;".to_string());
    selectors.register("-rotate-1", "--transform-rotate: -1deg;".to_string());
    selectors.register("translate-x-0", "--transform-translate-x: 0;".to_string());
    selectors.register(
        "translate-x-0.5",
        "--transform-translate-x: 0.125rem;".to_string(),
    );
    selectors.register(
        "translate-x-1",
        "--transform-translate-x: 0.25rem;".to_string(),
    );
    selectors.register(
        "translate-x-1.5",
        "--transform-translate-x: 0.375rem;".to_string(),
    );
    selectors.register(
        "translate-x-2",
        "--transform-translate-x: 0.5rem;".to_string(),
    );
    selectors.register(
        "translate-x-2.5",
        "--transform-translate-x: 0.625rem;".to_string(),
    );
    selectors.register(
        "translate-x-3",
        "--transform-translate-x: 0.75rem;".to_string(),
    );
    selectors.register(
        "translate-x-3.5",
        "--transform-translate-x: 0.875rem;".to_string(),
    );
    selectors.register(
        "translate-x-4",
        "--transform-translate-x: 1rem;".to_string(),
    );
    selectors.register(
        "translate-x-5",
        "--transform-translate-x: 1.25rem;".to_string(),
    );
    selectors.register(
        "translate-x-6",
        "--transform-translate-x: 1.5rem;".to_string(),
    );
    selectors.register(
        "translate-x-7",
        "--transform-translate-x: 1.75rem;".to_string(),
    );
    selectors.register(
        "translate-x-8",
        "--transform-translate-x: 2rem;".to_string(),
    );
    selectors.register(
        "translate-x-9",
        "--transform-translate-x: 2.25rem;".to_string(),
    );
    selectors.register(
        "translate-x-10",
        "--transform-translate-x: 2.5rem;".to_string(),
    );
    selectors.register(
        "translate-x-11",
        "--transform-translate-x: 2.75rem;".to_string(),
    );
    selectors.register(
        "translate-x-12",
        "--transform-translate-x: 3rem;".to_string(),
    );
    selectors.register(
        "translate-x-14",
        "--transform-translate-x: 3.5rem;".to_string(),
    );
    selectors.register(
        "translate-x-16",
        "--transform-translate-x: 4rem;".to_string(),
    );
    selectors.register(
        "translate-x-20",
        "--transform-translate-x: 5rem;".to_string(),
    );
    selectors.register(
        "translate-x-24",
        "--transform-translate-x: 6rem;".to_string(),
    );
    selectors.register(
        "translate-x-28",
        "--transform-translate-x: 7rem;".to_string(),
    );
    selectors.register(
        "translate-x-32",
        "--transform-translate-x: 8rem;".to_string(),
    );
    selectors.register(
        "translate-x-36",
        "--transform-translate-x: 8rem;".to_string(),
    );
    selectors.register(
        "translate-x-40",
        "--transform-translate-x: 10rem;".to_string(),
    );
    selectors.register(
        "translate-x-44",
        "--transform-translate-x: 11rem;".to_string(),
    );
    selectors.register(
        "translate-x-48",
        "--transform-translate-x: 12rem;".to_string(),
    );
    selectors.register(
        "translate-x-52",
        "--transform-translate-x: 13rem;".to_string(),
    );
    selectors.register(
        "translate-x-56",
        "--transform-translate-x: 14rem;".to_string(),
    );
    selectors.register(
        "translate-x-60",
        "--transform-translate-x: 15rem;".to_string(),
    );
    selectors.register(
        "translate-x-64",
        "--transform-translate-x: 16rem;".to_string(),
    );
    selectors.register(
        "translate-x-72",
        "--transform-translate-x: 18rem;".to_string(),
    );
    selectors.register(
        "translate-x-80",
        "--transform-translate-x: 20rem;".to_string(),
    );
    selectors.register(
        "translate-x-96",
        "--transform-translate-x: 24rem;".to_string(),
    );
    selectors.register(
        "translate-x-px",
        "--transform-translate-x: 1px;".to_string(),
    );
    selectors.register(
        "translate-x-1/2",
        "--transform-translate-x: 50%;".to_string(),
    );
    selectors.register(
        "translate-x-1/3",
        "--transform-translate-x: 33.333333%;".to_string(),
    );
    selectors.register(
        "translate-x-2/3",
        "--transform-translate-x: 66.6666666%;".to_string(),
    );
    selectors.register(
        "translate-x-1/4",
        "--transform-translate-x: 25%;".to_string(),
    );
    selectors.register(
        "translate-x-2/4",
        "--transform-translate-x: 50%;".to_string(),
    );
    selectors.register(
        "translate-x-3/4",
        "--transform-translate-x: 75%;".to_string(),
    );
    selectors.register(
        "translate-x-full",
        "--transform-translate-x: 100%;".to_string(),
    );
    selectors.register("-translate-x-0", "--transform-translate-x: 0;".to_string());
    selectors.register(
        "-translate-x-0.5",
        "--transform-translate-x: -0.125rem;".to_string(),
    );
    selectors.register(
        "-translate-x-1",
        "--transform-translate-x: -0.25rem;".to_string(),
    );
    selectors.register(
        "-translate-x-1.5",
        "--transform-translate-x: -0.375rem;".to_string(),
    );
    selectors.register(
        "-translate-x-2",
        "--transform-translate-x: -0.5rem;".to_string(),
    );
    selectors.register(
        "-translate-x-2.5",
        "--transform-translate-x: -0.625rem;".to_string(),
    );
    selectors.register(
        "-translate-x-3",
        "--transform-translate-x: -0.75rem;".to_string(),
    );
    selectors.register(
        "-translate-x-3.5",
        "--transform-translate-x: -0.875rem;".to_string(),
    );
    selectors.register(
        "-translate-x-4",
        "--transform-translate-x: -1rem;".to_string(),
    );
    selectors.register(
        "-translate-x-5",
        "--transform-translate-x: -1.25rem;".to_string(),
    );
    selectors.register(
        "-translate-x-6",
        "--transform-translate-x: -1.5rem;".to_string(),
    );
    selectors.register(
        "-translate-x-7",
        "--transform-translate-x: -1.75rem;".to_string(),
    );
    selectors.register(
        "-translate-x-8",
        "--transform-translate-x: -2rem;".to_string(),
    );
    selectors.register(
        "-translate-x-9",
        "--transform-translate-x: -2.25rem;".to_string(),
    );
    selectors.register(
        "-translate-x-10",
        "--transform-translate-x: -2.5rem;".to_string(),
    );
    selectors.register(
        "-translate-x-11",
        "--transform-translate-x: -2.75rem;".to_string(),
    );
    selectors.register(
        "-translate-x-12",
        "--transform-translate-x: -3rem;".to_string(),
    );
    selectors.register(
        "-translate-x-14",
        "--transform-translate-x: -3.5rem;".to_string(),
    );
    selectors.register(
        "-translate-x-16",
        "--transform-translate-x: -4rem;".to_string(),
    );
    selectors.register(
        "-translate-x-20",
        "--transform-translate-x: -5rem;".to_string(),
    );
    selectors.register(
        "-translate-x-24",
        "--transform-translate-x: -6rem;".to_string(),
    );
    selectors.register(
        "-translate-x-28",
        "--transform-translate-x: -7rem;".to_string(),
    );
    selectors.register(
        "-translate-x-32",
        "--transform-translate-x: -8rem;".to_string(),
    );
    selectors.register(
        "-translate-x-36",
        "--transform-translate-x: -8rem;".to_string(),
    );
    selectors.register(
        "-translate-x-40",
        "--transform-translate-x: -10rem;".to_string(),
    );
    selectors.register(
        "-translate-x-44",
        "--transform-translate-x: -11rem;".to_string(),
    );
    selectors.register(
        "-translate-x-48",
        "--transform-translate-x: -12rem;".to_string(),
    );
    selectors.register(
        "-translate-x-52",
        "--transform-translate-x: -13rem;".to_string(),
    );
    selectors.register(
        "-translate-x-56",
        "--transform-translate-x: -14rem;".to_string(),
    );
    selectors.register(
        "-translate-x-60",
        "--transform-translate-x: -15rem;".to_string(),
    );
    selectors.register(
        "-translate-x-64",
        "--transform-translate-x: -16rem;".to_string(),
    );
    selectors.register(
        "-translate-x-72",
        "--transform-translate-x: -18rem;".to_string(),
    );
    selectors.register(
        "-translate-x-80",
        "--transform-translate-x: -20rem;".to_string(),
    );
    selectors.register(
        "-translate-x-96",
        "--transform-translate-x: -24rem;".to_string(),
    );
    selectors.register(
        "-translate-x-px",
        "--transform-translate-x: -1px;".to_string(),
    );
    selectors.register(
        "-translate-x-1/2",
        "--transform-translate-x: -50%;".to_string(),
    );
    selectors.register(
        "-translate-x-1/3",
        "--transform-translate-x: -33.333333%;".to_string(),
    );
    selectors.register(
        "-translate-x-2/3",
        "--transform-translate-x: -66.6666666%;".to_string(),
    );
    selectors.register(
        "-translate-x-1/4",
        "--transform-translate-x: -25%;".to_string(),
    );
    selectors.register(
        "-translate-x-2/4",
        "--transform-translate-x: -50%;".to_string(),
    );
    selectors.register(
        "-translate-x-3/4",
        "--transform-translate-x: -75%;".to_string(),
    );
    selectors.register(
        "-translate-x-full",
        "--transform-translate-x: -100%;".to_string(),
    );
    selectors.register("translate-y-0", "--transform-translate-y: 0;".to_string());
    selectors.register(
        "translate-y-0.5",
        "--transform-translate-y: 0.125rem;".to_string(),
    );
    selectors.register(
        "translate-y-1",
        "--transform-translate-y: 0.25rem;".to_string(),
    );
    selectors.register(
        "translate-y-1.5",
        "--transform-translate-y: 0.375rem;".to_string(),
    );
    selectors.register(
        "translate-y-2",
        "--transform-translate-y: 0.5rem;".to_string(),
    );
    selectors.register(
        "translate-y-2.5",
        "--transform-translate-y: 0.625rem;".to_string(),
    );
    selectors.register(
        "translate-y-3",
        "--transform-translate-y: 0.75rem;".to_string(),
    );
    selectors.register(
        "translate-y-3.5",
        "--transform-translate-y: 0.875rem;".to_string(),
    );
    selectors.register(
        "translate-y-4",
        "--transform-translate-y: 1rem;".to_string(),
    );
    selectors.register(
        "translate-y-5",
        "--transform-translate-y: 1.25rem;".to_string(),
    );
    selectors.register(
        "translate-y-6",
        "--transform-translate-y: 1.5rem;".to_string(),
    );
    selectors.register(
        "translate-y-7",
        "--transform-translate-y: 1.75rem;".to_string(),
    );
    selectors.register(
        "translate-y-8",
        "--transform-translate-y: 2rem;".to_string(),
    );
    selectors.register(
        "translate-y-9",
        "--transform-translate-y: 2.25rem;".to_string(),
    );
    selectors.register(
        "translate-y-10",
        "--transform-translate-y: 2.5rem;".to_string(),
    );
    selectors.register(
        "translate-y-11",
        "--transform-translate-y: 2.75rem;".to_string(),
    );
    selectors.register(
        "translate-y-12",
        "--transform-translate-y: 3rem;".to_string(),
    );
    selectors.register(
        "translate-y-14",
        "--transform-translate-y: 3.5rem;".to_string(),
    );
    selectors.register(
        "translate-y-16",
        "--transform-translate-y: 4rem;".to_string(),
    );
    selectors.register(
        "translate-y-20",
        "--transform-translate-y: 5rem;".to_string(),
    );
    selectors.register(
        "translate-y-24",
        "--transform-translate-y: 6rem;".to_string(),
    );
    selectors.register(
        "translate-y-28",
        "--transform-translate-y: 7rem;".to_string(),
    );
    selectors.register(
        "translate-y-32",
        "--transform-translate-y: 8rem;".to_string(),
    );
    selectors.register("translate-y-36", "translate".to_string());
    selectors.register(
        "translate-y-40",
        "--transform-translate-y: 10rem;".to_string(),
    );
    selectors.register(
        "translate-y-44",
        "--transform-translate-y: 11rem;".to_string(),
    );
    selectors.register(
        "translate-y-48",
        "--transform-translate-y: 12rem;".to_string(),
    );
    selectors.register(
        "translate-y-52",
        "--transform-translate-y: 13rem;".to_string(),
    );
    selectors.register(
        "translate-y-56",
        "--transform-translate-y: 14rem;".to_string(),
    );
    selectors.register(
        "translate-y-60",
        "--transform-translate-y: 15rem;".to_string(),
    );
    selectors.register(
        "translate-y-64",
        "--transform-translate-y: 16rem;".to_string(),
    );
    selectors.register(
        "translate-y-72",
        "--transform-translate-y: 18rem;".to_string(),
    );
    selectors.register(
        "translate-y-80",
        "--transform-translate-y: 20rem;".to_string(),
    );
    selectors.register(
        "translate-y-96",
        "--transform-translate-y: 24rem;".to_string(),
    );
    selectors.register(
        "translate-y-px",
        "--transform-translate-y: 1px;".to_string(),
    );
    selectors.register(
        "translate-y-1/2",
        "--transform-translate-y: 50%;".to_string(),
    );
    selectors.register(
        "translate-y-1/3",
        "--transform-translate-y: 33.333333%;".to_string(),
    );
    selectors.register(
        "translate-y-2/3",
        "--transform-translate-y: 66.6666666%;".to_string(),
    );
    selectors.register(
        "translate-y-1/4",
        "--transform-translate-y: 25%;".to_string(),
    );
    selectors.register(
        "translate-y-2/4",
        "--transform-translate-y: 50%;".to_string(),
    );
    selectors.register(
        "translate-y-3/4",
        "--transform-translate-y: 75%;".to_string(),
    );
    selectors.register(
        "translate-y-full",
        "--transform-translate-y: 100%;".to_string(),
    );
    selectors.register("-translate-y-0", "--transform-translate-y: 0;".to_string());
    selectors.register(
        "-translate-y-0.5",
        "--transform-translate-y: -0.125rem;".to_string(),
    );
    selectors.register(
        "-translate-y-1",
        "--transform-translate-y: -0.25rem;".to_string(),
    );
    selectors.register(
        "-translate-y-1.5",
        "--transform-translate-y: -0.375rem;".to_string(),
    );
    selectors.register(
        "-translate-y-2",
        "--transform-translate-y: -0.5rem;".to_string(),
    );
    selectors.register(
        "-translate-y-2.5",
        "--transform-translate-y: -0.625rem;".to_string(),
    );
    selectors.register(
        "-translate-y-3",
        "--transform-translate-y: -0.75rem;".to_string(),
    );
    selectors.register(
        "-translate-y-3.5",
        "--transform-translate-y: -0.875rem;".to_string(),
    );
    selectors.register(
        "-translate-y-4",
        "--transform-translate-y: -1rem;".to_string(),
    );
    selectors.register(
        "-translate-y-5",
        "--transform-translate-y: -1.25rem;".to_string(),
    );
    selectors.register(
        "-translate-y-6",
        "--transform-translate-y: -1.5rem;".to_string(),
    );
    selectors.register(
        "-translate-y-7",
        "--transform-translate-y: -1.75rem;".to_string(),
    );
    selectors.register(
        "-translate-y-8",
        "--transform-translate-y: -2rem;".to_string(),
    );
    selectors.register(
        "-translate-y-9",
        "--transform-translate-y: -2.25rem;".to_string(),
    );
    selectors.register(
        "-translate-y-10",
        "--transform-translate-y: -2.5rem;".to_string(),
    );
    selectors.register(
        "-translate-y-11",
        "--transform-translate-y: -2.75rem;".to_string(),
    );
    selectors.register(
        "-translate-y-12",
        "--transform-translate-y: -3rem;".to_string(),
    );
    selectors.register(
        "-translate-y-14",
        "--transform-translate-y: -3.5rem;".to_string(),
    );
    selectors.register(
        "-translate-y-16",
        "--transform-translate-y: -4rem;".to_string(),
    );
    selectors.register(
        "-translate-y-20",
        "--transform-translate-y: -5rem;".to_string(),
    );
    selectors.register(
        "-translate-y-24",
        "--transform-translate-y: -6rem;".to_string(),
    );
    selectors.register(
        "-translate-y-28",
        "--transform-translate-y: -7rem;".to_string(),
    );
    selectors.register(
        "-translate-y-32",
        "--transform-translate-y: -8rem;".to_string(),
    );
    selectors.register(
        "-translate-y-36",
        "--transform-translate-y: -8rem;".to_string(),
    );
    selectors.register(
        "-translate-y-40",
        "--transform-translate-y: -10rem;".to_string(),
    );
    selectors.register(
        "-translate-y-44",
        "--transform-translate-y: -11rem;".to_string(),
    );
    selectors.register(
        "-translate-y-48",
        "--transform-translate-y: -12rem;".to_string(),
    );
    selectors.register(
        "-translate-y-52",
        "--transform-translate-y: -13rem;".to_string(),
    );
    selectors.register(
        "-translate-y-56",
        "--transform-translate-y: -14rem;".to_string(),
    );
    selectors.register(
        "-translate-y-60",
        "--transform-translate-y: -15rem;".to_string(),
    );
    selectors.register(
        "-translate-y-64",
        "--transform-translate-y: -16rem;".to_string(),
    );
    selectors.register(
        "-translate-y-72",
        "--transform-translate-y: -18rem;".to_string(),
    );
    selectors.register(
        "-translate-y-80",
        "--transform-translate-y: -20rem;".to_string(),
    );
    selectors.register(
        "-translate-y-96",
        "--transform-translate-y: -24rem;".to_string(),
    );
    selectors.register(
        "-translate-y-px",
        "--transform-translate-y: -1px;".to_string(),
    );
    selectors.register(
        "-translate-y-1/2",
        "--transform-translate-y: -50%;".to_string(),
    );
    selectors.register(
        "-translate-y-1/3",
        "--transform-translate-y: -33.333333%;".to_string(),
    );
    selectors.register(
        "-translate-y-2/3",
        "--transform-translate-y: -66.6666666%;".to_string(),
    );
    selectors.register(
        "-translate-y-1/4",
        "--transform-translate-y: -25%;".to_string(),
    );
    selectors.register(
        "-translate-y-2/4",
        "--transform-translate-y: -50%;".to_string(),
    );
    selectors.register(
        "-translate-y-3/4",
        "--transform-translate-y: -75%;".to_string(),
    );
    selectors.register(
        "-translate-y-full",
        "--transform-translate-y: -100%;".to_string(),
    );
    selectors.register("skew-x-0", "--transform-skew-x: 0;".to_string());
    selectors.register("skew-x-1", "--transform-skew-x: 1deg;".to_string());
    selectors.register("skew-x-2", "--transform-skew-x: 2deg;".to_string());
    selectors.register("skew-x-3", "--transform-skew-x: 3deg;".to_string());
    selectors.register("skew-x-6", "--transform-skew-x: 6deg;".to_string());
    selectors.register("skew-x-12", "--transform-skew-x: 12deg;".to_string());
    selectors.register("-skew-x-12", "--transform-skew-x: -12deg;".to_string());
    selectors.register("-skew-x-6", "--transform-skew-x: -6deg;".to_string());
    selectors.register("-skew-x-3", "--transform-skew-x: -3deg;".to_string());
    selectors.register("-skew-x-2", "--transform-skew-x: -2deg;".to_string());
    selectors.register("-skew-x-1", "--transform-skew-x: -1deg;".to_string());
    selectors.register("skew-y-0", "--transform-skew-y: 0deg;".to_string());
    selectors.register("skew-y-1", "--transform-skew-y: 1deg;".to_string());
    selectors.register("skew-y-2", "--transform-skew-y: 2deg;".to_string());
    selectors.register("skew-y-3", "--transform-skew-y: 3deg;".to_string());
    selectors.register("skew-y-6", "--transform-skew-y: 6deg;".to_string());
    selectors.register("skew-y-12", "--transform-skew-y: 12deg;".to_string());
    selectors.register("-skew-y-12", "--transform-skew-y: -12deg;".to_string());
    selectors.register("-skew-y-6", "--transform-skew-y: -6deg;".to_string());
    selectors.register("-skew-y-3", "--transform-skew-y: -3deg;".to_string());
    selectors.register("-skew-y-2", "--transform-skew-y: -2deg;".to_string());
    selectors.register("-skew-y-1", "--transform-skew-y: -1deg;".to_string());
}*/

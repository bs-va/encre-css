/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register(
        "grid-cols-1",
        "grid-template-columns: repeat(1, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-2",
        "grid-template-columns: repeat(2, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-3",
        "grid-template-columns: repeat(3, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-4",
        "grid-template-columns: repeat(4, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-5",
        "grid-template-columns: repeat(5, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-6",
        "grid-template-columns: repeat(6, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-7",
        "grid-template-columns: repeat(7, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-8",
        "grid-template-columns: repeat(8, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-9",
        "grid-template-columns: repeat(9, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-10",
        "grid-template-columns: repeat(10, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-11",
        "grid-template-columns: repeat(11, minmax(0, 1fr));".to_string(),
    );
    selectors.register(
        "grid-cols-12",
        "grid-template-columns: repeat(12, minmax(0, 1fr));".to_string(),
    );
    selectors.register("grid-cols-none", "grid-template-columns: none;".to_string());
    selectors.register("col-auto", "".to_string());
    selectors.register("col-span-1", "".to_string());
    selectors.register("col-span-2", "".to_string());
    selectors.register("col-span-3", "".to_string());
    selectors.register("col-span-4", "".to_string());
    selectors.register("col-span-5", "".to_string());
    selectors.register("col-span-6", "".to_string());
    selectors.register("col-span-7", "".to_string());
    selectors.register("col-span-8", "".to_string());
    selectors.register("col-span-9", "".to_string());
    selectors.register("col-span-10", "".to_string());
    selectors.register("col-span-11", "".to_string());
    selectors.register("col-span-12", "".to_string());
    selectors.register("col-start-1", "".to_string());
    selectors.register("col-start-2", "".to_string());
    selectors.register("col-start-3", "".to_string());
    selectors.register("col-start-4", "".to_string());
    selectors.register("col-start-5", "".to_string());
    selectors.register("col-start-6", "".to_string());
    selectors.register("col-start-7", "".to_string());
    selectors.register("col-start-8", "".to_string());
    selectors.register("col-start-9", "".to_string());
    selectors.register("col-start-10", "".to_string());
    selectors.register("col-start-11", "".to_string());
    selectors.register("col-start-12", "".to_string());
    selectors.register("col-start-13", "".to_string());
    selectors.register("col-start-auto", "".to_string());
    selectors.register("col-end-1", "".to_string());
    selectors.register("col-end-2", "".to_string());
    selectors.register("col-end-3", "".to_string());
    selectors.register("col-end-4", "".to_string());
    selectors.register("col-end-5", "".to_string());
    selectors.register("col-end-6", "".to_string());
    selectors.register("col-end-7", "".to_string());
    selectors.register("col-end-8", "".to_string());
    selectors.register("col-end-9", "".to_string());
    selectors.register("col-end-10", "".to_string());
    selectors.register("col-end-11", "".to_string());
    selectors.register("col-end-12", "".to_string());
    selectors.register("col-end-13", "".to_string());
    selectors.register("col-end-auto", "".to_string());
    selectors.register("grid-rows-1", "".to_string());
    selectors.register("grid-rows-2", "".to_string());
    selectors.register("grid-rows-3", "".to_string());
    selectors.register("grid-rows-4", "".to_string());
    selectors.register("grid-rows-5", "".to_string());
    selectors.register("grid-rows-6", "".to_string());
    selectors.register("grid-rows-none", "".to_string());
    selectors.register("row-auto", "".to_string());
    selectors.register("row-span-1", "".to_string());
    selectors.register("row-span-2", "".to_string());
    selectors.register("row-span-3", "".to_string());
    selectors.register("row-span-4", "".to_string());
    selectors.register("row-span-5", "".to_string());
    selectors.register("row-span-6", "".to_string());
    selectors.register("row-start-1", "".to_string());
    selectors.register("row-start-2", "".to_string());
    selectors.register("row-start-3", "".to_string());
    selectors.register("row-start-4", "".to_string());
    selectors.register("row-start-5", "".to_string());
    selectors.register("row-start-6", "".to_string());
    selectors.register("row-start-7", "".to_string());
    selectors.register("row-start-auto", "".to_string());
    selectors.register("row-end-1", "".to_string());
    selectors.register("row-end-2", "".to_string());
    selectors.register("row-end-3", "".to_string());
    selectors.register("row-end-4", "".to_string());
    selectors.register("row-end-5", "".to_string());
    selectors.register("row-end-6", "".to_string());
    selectors.register("row-end-7", "".to_string());
    selectors.register("row-end-auto", "".to_string());
    selectors.register("gap-0", "gap: 0;".to_string());
    selectors.register("gap-0.5", "gap: 0.125rem;".to_string());
    selectors.register("gap-1", "gap: 0.25rem;".to_string());
    selectors.register("gap-1.5", "gap: 0.375rem;".to_string());
    selectors.register("gap-2", "gap: 0.5rem;".to_string());
    selectors.register("gap-2.5", "gap: 0.625rem;".to_string());
    selectors.register("gap-3", "gap: 0.75rem;".to_string());
    selectors.register("gap-3.5", "gap: 0.875rem;".to_string());
    selectors.register("gap-4", "gap: 1rem;".to_string());
    selectors.register("gap-5", "gap: 1.25rem;".to_string());
    selectors.register("gap-6", "gap: 1.5rem;".to_string());
    selectors.register("gap-8", "gap: 2rem;".to_string());
    selectors.register("gap-10", "gap: 2.5rem;".to_string());
    selectors.register("gap-11", "gap: 2.75rem;".to_string());
    selectors.register("gap-12", "gap: 3rem;".to_string());
    selectors.register("gap-14", "gap: 3.5rem;".to_string());
    selectors.register("gap-16", "gap: 4rem;".to_string());
    selectors.register("gap-20", "gap: 5rem;".to_string());
    selectors.register("gap-24", "gap: 6rem;".to_string());
    selectors.register("gap-28", "gap: 7rem;".to_string());
    selectors.register("gap-32", "gap: 8rem;".to_string());
    selectors.register("gap-36", "gap: 9rem;".to_string());
    selectors.register("gap-40", "gap: 10rem;".to_string());
    selectors.register("gap-44", "gap: 11rem;".to_string());
    selectors.register("gap-48", "gap: 12rem;".to_string());
    selectors.register("gap-52", "gap: 13rem;".to_string());
    selectors.register("gap-56", "gap: 14rem;".to_string());
    selectors.register("gap-64", "gap: 16rem;".to_string());
    selectors.register("gap-72", "gap: 18rem;".to_string());
    selectors.register("gap-80", "gap: 20rem;".to_string());
    selectors.register("gap-96", "gap: 24rem;".to_string());
    selectors.register("gap-px", "gap: 1px;".to_string());
    selectors.register("gap-x-0", "".to_string());
    selectors.register("gap-x-0.5", "".to_string());
    selectors.register("gap-x-1", "".to_string());
    selectors.register("gap-x-1.5", "".to_string());
    selectors.register("gap-x-2", "".to_string());
    selectors.register("gap-x-2.5", "".to_string());
    selectors.register("gap-x-3", "".to_string());
    selectors.register("gap-x-3.5", "".to_string());
    selectors.register("gap-x-4", "".to_string());
    selectors.register("gap-x-5", "".to_string());
    selectors.register("gap-x-6", "".to_string());
    selectors.register("gap-x-8", "".to_string());
    selectors.register("gap-x-10", "".to_string());
    selectors.register("gap-x-11", "".to_string());
    selectors.register("gap-x-12", "".to_string());
    selectors.register("gap-x-14", "".to_string());
    selectors.register("gap-x-16", "".to_string());
    selectors.register("gap-x-20", "".to_string());
    selectors.register("gap-x-24", "".to_string());
    selectors.register("gap-x-28", "".to_string());
    selectors.register("gap-x-32", "".to_string());
    selectors.register("gap-x-36", "".to_string());
    selectors.register("gap-x-40", "".to_string());
    selectors.register("gap-x-44", "".to_string());
    selectors.register("gap-x-48", "".to_string());
    selectors.register("gap-x-52", "".to_string());
    selectors.register("gap-x-56", "".to_string());
    selectors.register("gap-x-64", "".to_string());
    selectors.register("gap-x-72", "".to_string());
    selectors.register("gap-x-80", "".to_string());
    selectors.register("gap-x-96", "".to_string());
    selectors.register("gap-x-px", "".to_string());
    selectors.register("gap-y-0", "".to_string());
    selectors.register("gap-y-0.5", "".to_string());
    selectors.register("gap-y-1", "".to_string());
    selectors.register("gap-y-1.5", "".to_string());
    selectors.register("gap-y-2", "".to_string());
    selectors.register("gap-y-2.5", "".to_string());
    selectors.register("gap-y-3", "".to_string());
    selectors.register("gap-y-3.5", "".to_string());
    selectors.register("gap-y-4", "".to_string());
    selectors.register("gap-y-5", "".to_string());
    selectors.register("gap-y-6", "".to_string());
    selectors.register("gap-y-8", "".to_string());
    selectors.register("gap-y-10", "".to_string());
    selectors.register("gap-y-11", "".to_string());
    selectors.register("gap-y-12", "".to_string());
    selectors.register("gap-y-14", "".to_string());
    selectors.register("gap-y-16", "".to_string());
    selectors.register("gap-y-20", "".to_string());
    selectors.register("gap-y-24", "".to_string());
    selectors.register("gap-y-28", "".to_string());
    selectors.register("gap-y-32", "".to_string());
    selectors.register("gap-y-36", "".to_string());
    selectors.register("gap-y-40", "".to_string());
    selectors.register("gap-y-44", "".to_string());
    selectors.register("gap-y-48", "".to_string());
    selectors.register("gap-y-52", "".to_string());
    selectors.register("gap-y-56", "".to_string());
    selectors.register("gap-y-64", "".to_string());
    selectors.register("gap-y-72", "".to_string());
    selectors.register("gap-y-80", "".to_string());
    selectors.register("gap-y-96", "".to_string());
    selectors.register("gap-y-px", "".to_string());
    selectors.register("grid-flow-row", "".to_string());
    selectors.register("grid-flow-col", "".to_string());
    selectors.register("grid-flow-row-dense", "".to_string());
    selectors.register("grid-flow-col-dense", "".to_string());
    selectors.register("auto-cols-auto", "".to_string());
    selectors.register("auto-cols-min", "".to_string());
    selectors.register("auto-cols-max", "".to_string());
    selectors.register("auto-cols-fr", "".to_string());
    selectors.register("auto-rows-auto", "".to_string());
    selectors.register("auto-rows-min", "".to_string());
    selectors.register("auto-rows-max", "".to_string());
    selectors.register("auto-rows-fr", "".to_string());
}*/

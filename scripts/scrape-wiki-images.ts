// small utility to scrape map images off of the tf2 wiki
// `deno run --allow-net ./scripts/scrape-wiki-images.ts`
// the output should go in src/constants.rs
import { DOMParser } from "https://deno.land/x/deno_dom/deno-dom-wasm.ts";

const WIKI = "https://wiki.teamfortress.com";

const document = new DOMParser().parseFromString(
	await (await fetch(`${WIKI}/wiki/List_of_maps`)).text(),
	"text/html",
);
const rows = document.querySelectorAll("table > tbody > tr");
const pairs = [];

for (const row of rows) {
	const map = row.querySelector("td > code");
	const img = row.querySelector("td:first-child > a > img");

	if (!map || !img) continue;

	const mapText = map.textContent;
	const imgLink = `${WIKI}${img.getAttribute("src").replace("150px", "1200px")}`;

	pairs.push(`	("${mapText}", "${imgLink}"),`);
}

console.log(`const MAP_IMAGES: [(&str, &str); ${pairs.length}] = [`);
pairs.forEach((pair) => console.log(pair));
console.log(`];`);

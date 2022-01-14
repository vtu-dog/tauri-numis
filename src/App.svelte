<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type Product from "./product";
	import ProductCarousel from "./ProductCarousel.svelte";

	document.addEventListener("contextmenu", (event) => event.preventDefault());
	document.addEventListener("selectstart", (event) => event.preventDefault());

	const productsPromise = async () => {
		let blob = await invoke("retrieve_products", {});
		let products: [Product] = JSON.parse(blob.toString());

		return products;
	};
</script>

<main>
	{#await productsPromise() then products}
		<h4>
			{#if products.length === 1}
				W ofercie CTMP Numis pojawił się nowy przedmiot
			{:else}
				W ofercie CTMP Numis pojawiły się nowe przedmioty
			{/if}
		</h4>

		<ProductCarousel {products} />
	{/await}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: none;
		margin: 0 auto;
	}

	:global(h4) {
		color: #7e2102;
		text-transform: uppercase;
		padding: 15px;
		margin-bottom: 15px !important;
	}
</style>

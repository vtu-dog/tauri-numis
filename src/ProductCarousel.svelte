<script lang="ts">
    import {
        Button,
        Card,
        CardBody,
        CardFooter,
        CardHeader,
        CardSubtitle,
        CardText,
        CardTitle,
    } from "sveltestrap";

    import Carousel from "svelte-carousel";
    import { open } from "@tauri-apps/api/shell";

    import SwapImage from "./SwapImage.svelte";
    import type Product from "./product";

    export let products: [Product];

    async function openInBrowser(href: string) {
        await open(href);
    }
</script>

<Carousel>
    {#each products as product}
        <Card>
            <CardHeader>
                <CardTitle>{product.title}</CardTitle>
            </CardHeader>
            <CardBody>
                <SwapImage
                    obverse={product.obverse_img_href}
                    reverse={product.reverse_img_href}
                />
                <CardSubtitle>{product.category}</CardSubtitle>
                <CardText>{product.price}</CardText>
            </CardBody>
            <CardFooter>
                <Button
                    on:click={async (_e) => {
                        await openInBrowser(product.href);
                    }}>Do sklepu</Button
                >
            </CardFooter>
        </Card>
    {/each}
</Carousel>

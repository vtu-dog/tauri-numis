<script lang="ts">
    export let obverse: string;
    export let reverse: string;

    let alt = "obverse";
    let src = obverse;

    function handleMouseOver(_e: any) {
        alt = "reverse";
        src = reverse;
    }
    function handleMouseOut(_e: any) {
        alt = "obverse";
        src = obverse;
    }
</script>

<div style="width: 358px; height: 354px;">
    <img
        {src}
        {alt}
        style="max-width: 358px; max-height: 354px; width: auto;"
        on:mouseover={handleMouseOver}
        on:focus={() => {}}
        on:mouseout={handleMouseOut}
        on:blur={() => {}}
    />
</div>

<style>
    :global(img) {
        width: 100%;
        height: auto;
        padding: 10px 10px 15px 10px;
    }
</style>

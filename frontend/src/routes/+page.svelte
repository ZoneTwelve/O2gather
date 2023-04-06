<script>	
	import product_view from '$lib/images/product.png';
	import Product from './Products.svelte';
</script>

<svelte:head>
	<title>Hotdog on sale</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="the-product">
			<picture>
				<img src={product_view} alt="The Hotest dog" />
				<div class="description">
					<small>Hotdog series</small>
					<span class="h1">The hotdog</span>
					<span class="text">The fastest dog we hot</span>
				</div>
			</picture>
		</span>
</section>

<Product/>

<style>
	section {
		display: flex;
		/* flex-direction: column; */
		justify-content: center;
		/* align-items: center; */
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.the-product > picture > img {
		width: 100%;
		max-width: 600px;
	}

	.the-product > picture > div {
		position: relative;
		left: 10rem;
		bottom: 50%;
	}

	.description > *{
		line-height: 2rem;
		display: block;
	}

	.description > small {
		font-size: 1rem;
		font-weight: bold;
	}
	.description > span {
		font-size: 1rem;
	}
	.description > .h1 {
		font-size: 2rem;
		font-weight: bold;
	}
	.description > .text {
		line-height: 4rem;
	}

</style>

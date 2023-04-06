<!-- loaf products -->
<script>
  import hotdog_m1 from '$lib/images/products/hotdog-m1.png';
  import hotdog_r1 from '$lib/images/products/hotdog-r1.png';
  import hotdog_v1 from '$lib/images/products/hotdog-v1.png';
  let products = [
    {
      name: 'Hotdog M1',
      price: 999,
      src: hotdog_m1,
    },
    {
      name: 'Hotdog R1',
      price: 1999,
      src: hotdog_r1,
    },
    {
      name: 'Hotdog V1',
      price: 2999,
      src: hotdog_v1,
    },
  ];
</script>

<div class="container">
  <!-- each -->
  <div class="products">
    {#each products as product}
    <div class="product">
      <img src={product.src} alt={product.name} />
      <div class="description">
        <span class="name">{product.name}</span>
        <span class="price">${product.price}</span>
      </div>
    </div>
    {/each}
  </div>
</div>

<style>

.container{
  display: flex;
  justify-content: center;
  text-align: center;
  /* width:500px; */
}

.products {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;
  margin: 0 auto;
  width: 100%;
  max-width: 600px;
  padding: 0 1rem;
}

.product{
  background: #fdfff5;
  border-radius: 10px;
  width: 150px;
  height: auto;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.product > .description{
  text-align: left;
  padding: 1rem;
}

.product > img {
  width:100px;
}
</style>

# All together
## requirement

### Frontend

- Svelte
- SvelteKit
- TypeScript
- Mock Service Worker
- Aslant + Prettier
- Vitest + Playwright

#### Progress

Layout design:
![Main page layout](frontend-main-page-layout.png)

### Backend

- Actix (Rust)
- Diesel (PG)

#### Progress

Make an server can handle request and respose
Provide a virtual product with a special structure
```rust
// just make two value different
{
  "product_name": "&'static str"
  "product_id": "u32"
  "counter": "i32"
} 
```

Not Regular API Standard
- GET /product
- GET /buy/1


### Infrastructure

- k8s
- istio

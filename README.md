# safetensors-view

```bash
# output safetensors metadata json file
cargo run --release -- /media/nvme/models/DeepSeekV3/model-00001-of-000163.safetensors
cat output.json
```
print
```json
{
  "__metadata__": {
    "format": "pt"
  },
  "model.embed_tokens.weight": {
    "dtype": "BF16",
    "shape": [
      129280,
      7168
    ],
    "data_offsets": [
      0,
      1853358080
    ]
  },
  "model.layers.0.self_attn.q_a_proj.weight": {
    "dtype": "F8_E4M3",
    "shape": [
      1536,
      7168
    ],
    "data_offsets": [
      1853358080,
      1864368128
    ]
  },
  "model.layers.0.self_attn.q_a_proj.weight_scale_inv": {
    "dtype": "F32",
    "shape": [
      12,
      56
    ],
    "data_offsets": [
      1864368128,
      1864370816
    ]
  },
  "model.layers.0.self_attn.q_a_layernorm.weight": {
    "dtype": "BF16",
    "shape": [
      1536
    ],
    "data_offsets": [
      1864370816,
      1864373888
    ]
  },
  "model.layers.0.self_attn.q_b_proj.weight": {
    "dtype": "F8_E4M3",
    "shape": [
      24576,
      1536
    ],
    "data_offsets": [
      1864373888,
      1902122624
    ]
  },
  "model.layers.0.self_attn.q_b_proj.weight_scale_inv": {
    "dtype": "F32",
    "shape": [
      192,
      12
    ],
    "data_offsets": [
      1902122624,
      1902131840
    ]
  },
  "model.layers.0.self_attn.kv_a_proj_with_mqa.weight": {
    "dtype": "F8_E4M3",
    "shape": [
      576,
      7168
    ],
    "data_offsets": [
      1902131840,
      1906260608
    ]
  },
  "model.layers.0.self_attn.kv_a_proj_with_mqa.weight_scale_inv": {
    "dtype": "F32",
    "shape": [
      5,
      56
    ],
    "data_offsets": [
      1906260608,
      1906261728
    ]
  },
  "model.layers.0.self_attn.kv_a_layernorm.weight": {
    "dtype": "BF16",
    "shape": [
      512
    ],
    "data_offsets": [
      1906261728,
      1906262752
    ]

......
```

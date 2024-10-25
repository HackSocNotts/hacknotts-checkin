#set page(width: 72mm, height: auto, margin: 0pt)

#set text(
  font: "Departure Mono",
  size: 11pt,
  region: "GB"
)

#show heading: text.with(size: 2em)

#let reference = "{{reference}}"
#let name = "{{name}}"
#let discord = "{{discord}}"
#let pizza = "{{pizza}}"
#let release_title = "{{release_title}}"

#[
  #set align(center)
  = #release_title

  #image("folder_cat_high_contrast_gray.svg", width: 80%)
]

REFERENCE#h(1fr)#reference \
NAME#h(1fr)#name \

#pagebreak()

#[
  #set align(center)
  = PIZZA VOUCHER

  #name

  #image("folder_cat_high_contrast_gray.svg", width: 80%)

  #[
    #set text(size: 16.5pt)
    #pizza
  ]
]


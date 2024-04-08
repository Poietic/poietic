// Copyright 2024 Lech Mazur
//
// This file is part of Poietic.
//
// Poietic is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License, version 2,
// as published by the Free Software Foundation.
//
// Poietic is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Poietic. If not, see <https://www.gnu.org/licenses/>.

class ComponentParam {
  /** @type {string} */
  #name;
  /** @type {"text" | "number" | "composition-list"} */
  #type;
  constructor({ name, type }) {
    this.#name = name;
    this.#type = type;
  }
  get name() {
    return this.#name;
  }
  get type() {
    return this.#type;
  }
}

class Component {
  /** @type {string} */
  #name;
  /** @type {Map<string, ComponentParam>} */
  #params;
  constructor({ name, params }) {
    this.#name = name;
    this.#params = new Map(
      params.map((param) => [param.name, new ComponentParam(param)])
    );
  }
  get name() {
    return this.#name;
  }
  get params() {
    return this.#params;
  }
}

class CompositionBuilder {
  /** @type {Map<string, Component>} */
  #components;
  constructor(components) {
    this.#components = new Map(
      components.map((component) => [component.name, new Component(component)])
    );
  }
  get components() {
    return this.#components;
  }
}

/**
 * @param {Component} component
 * @returns {HTMLDivElement}
 */
function buildComponentNode(component) {
  const node = document.createElement("div");
  node.innerText = component.name;
  node.setAttribute("draggable", "true");
  node.addEventListener("dragstart", (event) => {
    event.dataTransfer.effectAllowed = "copy";
    event.dataTransfer.setData("text/poietic-component", component.name);
  });
  return node;
}

/**
 * @param {CompositionBuilder} compositionBuilder
 * @returns {HTMLDivElement}
 */
function constructComponentHolder(compositionBuilder) {
  const node = document.createElement("div");
  node.replaceChildren("Composition holder");
  node.addEventListener("dragover", (event) => {
    if (event.dataTransfer.types.includes("text/poietic-component")) {
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
    }
  });
  node.addEventListener("drop", (event) => {
    if (event.dataTransfer.types.includes("text/poietic-component")) {
      event.preventDefault();
      const data = event.dataTransfer.getData("text/poietic-component");
      const component = compositionBuilder.components.get(data);
      const componentNode = document.createElement("div");
      componentNode.innerText = component.name;
      node.replaceChildren("Composition holder", componentNode);
    }
  });
  return node;
}

/**
 * @param {HTMLDivElement} node
 * @param {CompositionBuilder} compositionBuilder
 */
function renderCompositionBuilder(node, compositionBuilder) {
  const componentsPanel = document.createElement("div");
  componentsPanel.replaceChildren(
    "Components",
    ...Array.from(compositionBuilder.components.values()).map(
      buildComponentNode
    )
  );
  const compositionHolder = constructComponentHolder(compositionBuilder);
  node.replaceChildren(
    "Composition builder",
    componentsPanel,
    compositionHolder
  );
}

const components = [
  {
    name: "poietic:Paragraph",
    params: [{ name: "content", type: "text" }],
  },
  {
    name: "poietic:Heading",
    params: [
      { name: "importance", type: "number" },
      { name: "text", type: "text" },
    ],
  },
  {
    name: "poietic:Link",
    params: [
      { name: "title", type: "text" },
      { name: "target", type: "text" },
    ],
  },
  {
    name: "poietic:CompositionList",
    params: [{ name: "children", type: "composition-list" }],
  },
  {
    name: "poietic:BasicPage",
    params: [
      { name: "title", type: "text" },
      { name: "nav_links", type: "composition-list" },
      { name: "content", type: "composition-list" },
    ],
  },
];

for (const compositionBuilderNode of document.getElementsByClassName(
  "poietic:CompositionBuilder"
)) {
  const compositionBuilder = new CompositionBuilder(components);
  renderCompositionBuilder(compositionBuilderNode, compositionBuilder);
}

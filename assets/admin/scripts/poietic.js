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
function constructComponentNode(component) {
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
function constructComponentsPanel(compositionBuilder) {
  const node = document.createElement("div");
  const caption = document.createElement("h3");
  caption.innerText = "Components";
  node.replaceChildren(
    caption,
    ...Array.from(compositionBuilder.components.values()).map(
      constructComponentNode
    )
  );
  return node;
}

/**
 * @param {CompositionBuilder} compositionBuilder
 * @param {ComponentParam} param
 * @returns {HTMLDivElement}
 */
function constructCompositionParamNode(compositionBuilder, param) {
  const node = document.createElement("div");
  node.setAttribute("poietic:type", "param");
  node.setAttribute("poietic:param_name", param.name);
  node.setAttribute("poietic:param_type", param.type);
  switch (param.type) {
    case "text":
    case "number": {
      const input = document.createElement("input");
      input.type = param.type;
      node.replaceChildren(param.name, input);
      break;
    }
    case "composition-list": {
      node.replaceChildren(
        param.name,
        ...constructCompositionListParam(compositionBuilder)
      );
      break;
    }
  }
  return node;
}

/**
 * @param {CompositionBuilder} compositionBuilder
 * @returns {HTMLDivElement[]}
 */
function constructCompositionListParam(compositionBuilder) {
  const compositionList = document.createElement("div");
  compositionList.setAttribute("poietic:type", "composition-list");
  const dropHandler = document.createElement("span");
  dropHandler.replaceChildren("[drop here]");
  dropHandler.addEventListener("dragover", handleDragOver);
  dropHandler.addEventListener("drop", handleDrop);
  return [dropHandler, compositionList];

  /**
   * @param {DragEvent} event
   */
  function handleDragOver(event) {
    if (event.dataTransfer.types.includes("text/poietic-component")) {
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
    }
  }

  /**
   * @param {DragEvent} event
   */
  function handleDrop(event) {
    if (event.dataTransfer.types.includes("text/poietic-component")) {
      event.preventDefault();
      const data = event.dataTransfer.getData("text/poietic-component");
      const component = compositionBuilder.components.get(data);
      const componentNode = constructCompositionNode(
        compositionBuilder,
        component
      );
      compositionList.appendChild(componentNode);
    }
  }
}

/**
 * @param {CompositionBuilder} compositionBuilder
 * @param {Component} component
 * @returns {HTMLDivElement}
 */
function constructCompositionNode(compositionBuilder, component) {
  const node = document.createElement("div");
  node.setAttribute("poietic:type", "composition");
  node.setAttribute("poietic:composition_component", component.name);
  node.replaceChildren(
    component.name,
    ...Array.from(component.params.values()).map((param) =>
      constructCompositionParamNode(compositionBuilder, param)
    )
  );
  return node;
}

/**
 * @param {[string, any][]} pairs
 * @returns {{[id: string]: any}}
 */
function collectObject(pairs) {
  const result = {};
  for (const pair of pairs) {
    result[pair[0]] = pair[1];
  }
  return result;
}

/**
 * @param {HTMLDivElement} compositionHolder
 */
function generateJson(compositionHolder) {
  /**
   * @param {"text" | "number" | "composition-list"} type
   * @param {HTMLElement} node
   */
  function scrapeParam(type, node) {
    switch (type) {
      case "number":
      case "text":
        return node.querySelector("input").value;
      case "composition-list": {
        const compositionList = Array.from(node.children)
          .filter(
            (node) => node.getAttribute("poietic:type") === "composition-list"
          )
          .shift();
        return Array.from(compositionList.children).map(scrapeComposition);
      }
    }
  }
  /**
   * @param {HTMLDivElement} node
   */
  function scrapeComposition(node) {
    const component = node.getAttribute("poietic:composition_component");
    const paramPairs = Array.from(node.children)
      .filter((node) => node.getAttribute("poietic:type") === "param")
      .map((node) => ({
        name: node.getAttribute("poietic:param_name"),
        type: node.getAttribute("poietic:param_type"),
        holder: node,
      }))
      .map((param) => [param.name, scrapeParam(param.type, param.holder)]);
    return { component, params: collectObject(paramPairs) };
  }
  const composition = Array.from(compositionHolder.children)
    .filter((node) => node.getAttribute("poietic:type") === "composition")
    .shift();
  const compositionJson = scrapeComposition(composition);
  alert(JSON.stringify(compositionJson));
}

/**
 * @param {CompositionBuilder} compositionBuilder
 * @returns {HTMLDivElement}
 */
function constructCompositionHolder(compositionBuilder) {
  const node = document.createElement("div");
  const caption = document.createElement("h3");
  const generateJsonButton = document.createElement("button");
  generateJsonButton.innerText = "Generate JSON";
  generateJsonButton.addEventListener("click", () => {
    if (hasComposition()) generateJson(node);
  });
  caption.innerText = "Composition holder";
  node.replaceChildren(caption, generateJsonButton);
  node.addEventListener("dragover", handleDragOver);
  node.addEventListener("drop", handleDrop);
  return node;

  function hasComposition() {
    return (
      Array.from(node.children).findIndex(
        (child) => child.getAttribute("poietic:type") === "composition"
      ) !== -1
    );
  }

  /**
   * @param {DragEvent} event
   */
  function handleDragOver(event) {
    if (
      event.dataTransfer.types.includes("text/poietic-component") &&
      !hasComposition()
    ) {
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
    }
  }

  /**
   * @param {DragEvent} event
   */
  function handleDrop(event) {
    if (
      event.dataTransfer.types.includes("text/poietic-component") &&
      !hasComposition()
    ) {
      event.preventDefault();
      const data = event.dataTransfer.getData("text/poietic-component");
      const component = compositionBuilder.components.get(data);
      const componentNode = constructCompositionNode(
        compositionBuilder,
        component
      );
      node.replaceChildren(caption, componentNode, generateJsonButton);
    }
  }
}

/**
 * @param {HTMLDivElement} node
 * @param {CompositionBuilder} compositionBuilder
 */
function renderCompositionBuilder(node, compositionBuilder) {
  const caption = document.createElement("h2");
  caption.innerText = "Composition builder";
  const componentsPanel = constructComponentsPanel(compositionBuilder);
  const compositionHolder = constructCompositionHolder(compositionBuilder);
  node.replaceChildren(caption, componentsPanel, compositionHolder);
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

/*
 * Copyright (c) [2024] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, contact SUSE LLC.
 *
 * To contact SUSE LLC about this file by physical or electronic mail, you may
 * find current contact information at www.suse.com.
 */

// @ts-check

import React, { useState } from "react";
import {
  Table,
  Thead,
  Tr,
  Th,
  Tbody,
  Td,
  ExpandableRowContent,
  RowSelectVariant,
} from "@patternfly/react-table";

/**
 * @typedef {import("@patternfly/react-table").TableProps} TableProps
 * @typedef {import("react").RefAttributes<HTMLTableElement>} HTMLTableProps
 */

/**
 * An object for sharing data across nested maps
 *
 * Since function arguments are always passed by value, an object passed by
 * sharing is needed for sharing data that might be mutated from different
 * places, as it is the case of the rowIndex prop here.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions#passing_arguments
 *
 * @typedef {object} SharedData
 * @property {number} rowIndex - The current row index, to be incremented each time a table row is generated.
 */

/**
 * @typedef {object} ExpandableSelectorColumn
 * @property {string} name - The column header text.
 * @property {(object) => React.ReactNode} value - A function receiving
 *   the item to work with and returning the column value.
 * @property {string} [classNames] - space-separated list of additional CSS class names.
 */

/**
 * Internal component for building the table header
 *
 * @param {object} props
 * @param {ExpandableSelectorColumn[]} props.columns
 */
const TableHeader = ({ columns }) => (
  <Thead noWrap>
    <Tr>
      <Th />
      <Th />
      {columns?.map((c, i) => (
        <Th key={i} className={c.classNames}>
          {c.name}
        </Th>
      ))}
    </Tr>
  </Thead>
);

/**
 * Helper function for ensuring a good value for ExpandableSelector#itemsSelected prop
 *
 * It logs information to console.error if given value does not match
 * expectations.
 *
 * @param {*} selection - The value to check.
 * @param {boolean} allowMultiple - Whether the returned collection can have
 *   more than one item
 * @return {Array} Empty array if given value is not valid. The first element if
 *   it is a collection with more than one but selector does not allow multiple.
 *   The original value otherwise.
 */
const sanitizeSelection = (selection, allowMultiple) => {
  if (!Array.isArray(selection)) {
    console.error("`itemSelected` prop must be an array. Ignoring given value", selection);
    return [];
  }

  if (!allowMultiple && selection.length > 1) {
    console.error(
      "`itemsSelected` prop can only have more than one item when selector `isMultiple`. " +
        "Using only the first element",
    );

    return [selection[0]];
  }

  return selection;
};

/**
 * Build a expandable table with selectable items.
 * @component
 *
 * @note It only accepts one nesting level.
 *
 * @typedef {object} ExpandableSelectorBaseProps
 * @property {ExpandableSelectorColumn[]} [columns=[]] - Collection of objects defining columns.
 * @property {boolean} [isMultiple=false] - Whether multiple selection is allowed.
 * @property {object[]} [items=[]] - Collection of items to be rendered.
 * @property {string} [itemIdKey="id"] - The key for retrieving the item id.
 * @property {(item: object) => Array<object>} [itemChildren=() => []] - Lookup method to retrieve children from given item.
 * @property {(item: object) => boolean} [itemSelectable=() => true] - Whether an item will be selectable or not.
 * @property {(item: object) => (string|undefined)} [itemClassNames=() => ""] - Callback that allows adding additional CSS class names to item row.
 * @property {object[]} [itemsSelected=[]] - Collection of selected items.
 * @property {any[]} [initialExpandedKeys=[]] - Ids of initially expanded items.
 * @property {(selection: Array<object>) => void} [onSelectionChange=noop] - Callback to be triggered when selection changes.
 *
 * @typedef {ExpandableSelectorBaseProps & TableProps & HTMLTableProps} ExpandableSelectorProps
 *
 * @param {ExpandableSelectorProps} props
 */
export default function ExpandableSelector({
  columns = [],
  isMultiple = false,
  items = [],
  itemIdKey = "id",
  itemChildren = () => [],
  itemSelectable = () => true,
  itemClassNames = () => "",
  itemsSelected = [],
  initialExpandedKeys = [],
  onSelectionChange,
  ...tableProps
}) {
  const [expandedItemsKeys, setExpandedItemsKeys] = useState(initialExpandedKeys);
  const selection = sanitizeSelection(itemsSelected, isMultiple);
  const isItemSelected = (item) => {
    const selected = selection.find((selectionItem) => {
      return (
        Object.hasOwn(selectionItem, itemIdKey) && selectionItem[itemIdKey] === item[itemIdKey]
      );
    });

    return selected !== undefined || selection.includes(item);
  };
  const isItemExpanded = (key) => expandedItemsKeys.includes(key);
  const toggleExpanded = (key) => {
    if (isItemExpanded(key)) {
      setExpandedItemsKeys(expandedItemsKeys.filter((k) => k !== key));
    } else {
      setExpandedItemsKeys([...expandedItemsKeys, key]);
    }
  };

  const updateSelection = (item) => {
    if (!isMultiple) {
      onSelectionChange([item]);
      return;
    }

    if (isItemSelected(item)) {
      onSelectionChange(selection.filter((i) => i !== item));
    } else {
      onSelectionChange([...selection, item]);
    }
  };

  /**
   * Render method for building the markup for an item child
   *
   * @param {object} item - The child to be rendered
   * @param {boolean} isExpanded - Whether the child should be shown or not
   * @param {SharedData} sharedData - An object holding shared data
   */
  const renderItemChild = (item, isExpanded, sharedData) => {
    const rowIndex = sharedData.rowIndex++;

    const selectProps = {
      rowIndex,
      onSelect: () => updateSelection(item),
      isSelected: isItemSelected(item),
      variant: isMultiple ? RowSelectVariant.checkbox : RowSelectVariant.radio,
    };

    return (
      <Tr key={rowIndex} isExpanded={isExpanded} className={itemClassNames(item)}>
        <Td />
        <Td select={itemSelectable(item) ? selectProps : undefined} />
        {columns?.map((column, index) => (
          <Td key={index} dataLabel={column.name} className={column.classNames}>
            <ExpandableRowContent>{column.value(item)}</ExpandableRowContent>
          </Td>
        ))}
      </Tr>
    );
  };

  /**
   * Render method for building the markup for item
   *
   * @param {object} item - The item to be rendered
   * @param {SharedData} sharedData - An object holding shared data
   */
  const renderItem = (item, sharedData) => {
    const itemKey = item[itemIdKey];
    const rowIndex = sharedData.rowIndex++;
    const children = itemChildren(item);
    const validChildren = Array.isArray(children) && children.length > 0;
    const expandProps = validChildren && {
      rowIndex,
      isExpanded: isItemExpanded(itemKey),
      onToggle: () => toggleExpanded(itemKey),
    };

    const selectProps = {
      rowIndex,
      onSelect: () => updateSelection(item),
      isSelected: isItemSelected(item),
      variant: isMultiple ? RowSelectVariant.checkbox : RowSelectVariant.radio,
    };

    const renderChildren = () => {
      if (!validChildren) return;

      return children.map((item) => renderItemChild(item, isItemExpanded(itemKey), sharedData));
    };

    // TODO: Add label to Tbody?
    return (
      <Tbody key={rowIndex} isExpanded={isItemExpanded(itemKey)}>
        <Tr className={itemClassNames(item)}>
          <Td expand={expandProps} />
          <Td select={itemSelectable(item) ? selectProps : undefined} />
          {columns?.map((column, index) => (
            <Td key={index} dataLabel={column.name} className={column.classNames}>
              {column.value(item)}
            </Td>
          ))}
        </Tr>
        {renderChildren()}
      </Tbody>
    );
  };

  // @see SharedData
  const sharedData = { rowIndex: 0 };

  const TableBody = () => items?.map((item) => renderItem(item, sharedData));

  return (
    <Table data-type="agama/expandable-selector" {...tableProps}>
      <TableHeader columns={columns} />
      <TableBody />
    </Table>
  );
}

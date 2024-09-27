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

import React from "react";
import { screen } from "@testing-library/react";
import { installerRender } from "~/test-utils";
import { PATHS } from "~/routes/products";
import { Product } from "~/types/software";
import ChangeProductLink from "./ChangeProductLink";

const tumbleweed: Product = {
  id: "Tumbleweed",
  name: "openSUSE Tumbleweed",
  icon: "tumbleweed.svg",
  description: "Tumbleweed description...",
};
const microos: Product = {
  id: "MicroOS",
  name: "openSUSE MicroOS",
  icon: "MicroOS.svg",
  description: "MicroOS description",
};

let mockUseProduct: { products: Product[]; selectedProduct?: Product };

jest.mock("~/queries/software", () => ({
  useProduct: () => mockUseProduct,
}));

describe("ChangeProductLink", () => {
  describe("when there is more than one product available", () => {
    beforeEach(() => {
      mockUseProduct = { products: [tumbleweed, microos] };
    });

    it("renders a link for navigating to product selection page", () => {
      installerRender(<ChangeProductLink />);
      const link = screen.getByRole("link", { name: "Change product" });
      expect(link).toHaveAttribute("href", PATHS.changeProduct);
    });
  });

  describe("when there is only one product available", () => {
    beforeEach(() => {
      mockUseProduct = { products: [tumbleweed] };
    });

    it("renders nothing", () => {
      const { container } = installerRender(<ChangeProductLink />);
      expect(container).toBeEmptyDOMElement();
    });
  });
});
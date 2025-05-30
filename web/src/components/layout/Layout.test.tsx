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
import { mockRoutes, installerRender } from "~/test-utils";
import Layout from "./Layout";

jest.mock("~/components/layout/Header", () => () => <div>Header Mock</div>);
jest.mock("~/components/layout/Sidebar", () => () => <div>Sidebar Mock</div>);
jest.mock("~/components/core/IssuesDrawer", () => () => <div>IssuesDrawer Mock</div>);
jest.mock("~/components/questions/Questions", () => () => <div>Questions Mock</div>);

describe("Layout", () => {
  it("renders a page with header and sidebar by default", () => {
    installerRender(<Layout />);
    screen.getByText("Header Mock");
    screen.getByText("Sidebar Mock");
  });

  it("does not render the header when mountHeader=false", () => {
    installerRender(<Layout mountHeader={false} />);
    expect(screen.queryByText("Header Mock")).toBeNull();
  });

  it("renders the skip to content when mountHeader=false and mountSkipToContent is not given", () => {
    installerRender(<Layout mountHeader={false} />);
    screen.getByRole("link", { name: "Skip to content" });
  });

  it("renders the skip to content when mountHeader=false and mountSkipToContent is true", () => {
    installerRender(<Layout mountHeader={false} mountSkipToContent />);
    screen.getByRole("link", { name: "Skip to content" });
  });

  it("does not render the skip to content when mountHeader=false and mountSkipToContent=false", () => {
    installerRender(<Layout mountHeader={false} mountSkipToContent={false} />);
    expect(screen.queryByRole("link", { name: "Skip to content" })).toBeNull();
  });

  it("does not render the sidebar when mountSidebar=false", () => {
    installerRender(<Layout mountSidebar={false} />);
    expect(screen.queryByText("Sidebar Mock")).toBeNull();
  });

  describe("when children are not given", () => {
    it("renders router <Outlet />", () => {
      installerRender(<Layout />);
      // NOTE: react-router-dom/Outlet is mock at src/test-utils
      screen.getByText("Outlet Content");
    });

    it("renders the questions component", () => {
      installerRender(<Layout />);
      screen.getByText("Questions Mock");
    });
  });

  describe("when children are given", () => {
    it("renders children instead of router <Outlet />", () => {
      installerRender(
        <Layout>
          <button>Dummy testing button</button>
        </Layout>,
      );
      screen.getByRole("button", { name: "Dummy testing button" });
      // NOTE: react-router-dom/Outlet is mock at src/test-utils
      expect(screen.queryByText("Outlet Content")).toBeNull();
    });

    it("renders the questions component", () => {
      installerRender(<Layout />);
      screen.getByText("Questions Mock");
    });
  });

  describe("at /login path", () => {
    beforeEach(() => {
      mockRoutes("/login");
    });

    it("does not render the questions component", () => {
      installerRender(<Layout />);
      expect(screen.queryByText("Questions Mock")).toBeNull();
    });
  });
});

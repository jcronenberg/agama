/*
 * Copyright (c) [2022] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License as published
 * by the Free Software Foundation.
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
import userEvent from "@testing-library/user-event";
import { authRender } from "./test-utils";

import DBusError from "./DBusError";

describe("DBusError", () => {
  it("includes a generic D-Bus connection problem message", async () => {
    authRender(<DBusError />);

    await screen.findByText(/Could not connect to the D-Bus service/i);
  });

  it("includes a button for reloading", async () => {
    authRender(<DBusError />);

    await screen.findByRole("button", { name: /Reload/i });
  });

  it("calls location.reload when user clicks on 'Reload'", async () => {
    authRender(<DBusError />);

    const reloadButton = await screen.findByRole("button", { name: /Reload/i });

    // Mock location.reload
    // https://remarkablemark.org/blog/2021/04/14/jest-mock-window-location-href
    const { location } = window;
    delete window.location;
    window.location = { reload: jest.fn() };

    userEvent.click(reloadButton);
    expect(window.location.reload).toHaveBeenCalled();

    // restore windows.location
    window.location = location;
  });
});

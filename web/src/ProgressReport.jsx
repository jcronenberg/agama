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

import React, { useState, useEffect } from "react";
import { useInstallerClient } from "./context/installer";

import { Progress, Stack, StackItem, Text } from "@patternfly/react-core";

const renderSubprogress = progress => (
  <Progress
    size="sm"
    measureLocation="none"
    aria-label="Secondary progress bar"
    value={Math.round((progress.substep / progress.substeps) * 100)}
  />
);

const ProgressReport = () => {
  const client = useInstallerClient();
  const [progress, setProgress] = useState({});

  useEffect(() => {
    return client.manager.onChange(changes => {
      if ("Progress" in changes) {
        const [title, steps, step, substeps, substep] = changes.Progress;
        setProgress({ title, steps, step, substeps, substep });
      }
    });
  }, []);

  if (!progress.steps) return <Text>Waiting for progress status...</Text>;

  const showSubsteps = !!progress.substeps && progress.substeps >= 0;
  const percentage = progress.steps === 0 ? 0 : Math.round((progress.step / progress.steps) * 100);

  return (
    <Stack hasGutter className="pf-u-w-100">
      <StackItem>
        <Progress aria-label="Main progress bar" title={progress.title} value={percentage} />
      </StackItem>

      <StackItem>{showSubsteps && renderSubprogress(progress)}</StackItem>
    </Stack>
  );
};

export default ProgressReport;
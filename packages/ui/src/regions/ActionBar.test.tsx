import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ActionBar } from "./ActionBar";

describe("ActionBar", () => {
  it("calls onAct with the tapped action's id, not its index", () => {
    const onAct = vi.fn();
    render(
      <ActionBar
        actions={[
          { id: 7, kind: "attack", label: "Slam" },
          { id: 12, kind: "retreat", label: "Retreat" },
        ]}
        onAct={onAct}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Retreat" }));
    expect(onAct).toHaveBeenCalledWith(12);
  });
});

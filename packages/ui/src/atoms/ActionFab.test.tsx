import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ActionFab } from "./ActionFab";

describe("ActionFab", () => {
  it("is reachable by its label, not by reading an emoji", () => {
    render(<ActionFab kind="attack" label="Slam" onClick={() => {}} />);
    expect(screen.getByRole("button", { name: "Slam" })).toBeInTheDocument();
  });

  it("fires onClick when tapped", () => {
    const onClick = vi.fn();
    render(<ActionFab kind="retreat" label="Retreat" onClick={onClick} />);
    fireEvent.click(screen.getByRole("button", { name: "Retreat" }));
    expect(onClick).toHaveBeenCalledOnce();
  });

  it("does not fire onClick while disabled", () => {
    const onClick = vi.fn();
    render(<ActionFab kind="evolve" label="Evolve" disabled onClick={onClick} />);
    fireEvent.click(screen.getByRole("button", { name: "Evolve" }));
    expect(onClick).not.toHaveBeenCalled();
  });

  it("carries a distinct icon for every action kind", () => {
    const kinds = ["attach", "retreat", "attack", "ability", "evolve"] as const;
    const seen = new Set<string>();
    for (const kind of kinds) {
      const { container, unmount } = render(<ActionFab kind={kind} label={kind} onClick={() => {}} />);
      seen.add(container.querySelector('[aria-hidden="true"]')!.textContent!);
      unmount();
    }
    expect(seen.size).toBe(kinds.length);
  });
});
